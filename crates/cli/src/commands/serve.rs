//! Serve an Buny source as a local source list.
use axum::{
	http::{header::USER_AGENT, Request, Response},
	Router,
};
use std::{net::SocketAddr, time::Duration};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::Level;

const MAX_SEARCH_DEPTH: u32 = 4;

fn find_bunpacks(
	dir: &std::path::Path,
	depth: u32,
	out: &mut Vec<std::path::PathBuf>,
) -> std::io::Result<()> {
	if depth > MAX_SEARCH_DEPTH {
		return Ok(());
	}
	for entry in std::fs::read_dir(dir)? {
		let path = entry?.path();
		if path.is_dir() {
			find_bunpacks(&path, depth + 1, out)?;
		} else if path.extension().is_some_and(|e| e == "bunpack") {
			out.push(path);
		}
	}
	Ok(())
}

pub async fn run(
	path: Option<std::path::PathBuf>,
	output_path: &std::path::PathBuf,
	port: u16,
) -> anyhow::Result<()> {
	let path = path.unwrap_or(std::env::current_dir()?);

	let files = if path.is_dir() {
		let mut found = Vec::new();
		find_bunpacks(&path, 0, &mut found)?;
		found
	} else {
		vec![path]
	};

	super::build::run(files, output_path, Some("Development Source List".into()))?;

	// enable logging
	tracing_subscriber::fmt()
		.with_target(false) // hide the `buny_cli::commands::serve:` text
		.with_max_level(Level::INFO)
		.init();

	// register static router for serving the output directory
	let app = Router::new()
		.fallback_service(ServeDir::new(output_path))
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(|req: &Request<axum::body::Body>| {
					// show the request user agent in the log
					let user_agent = req
						.headers()
						.get(USER_AGENT)
						.and_then(|h| h.to_str().ok())
						.unwrap_or("unknown");

					tracing::info_span!(
						"request",
						method = %req.method(),
						uri = %req.uri(),
						user_agent = %user_agent,
					)
				})
				.on_response(
					|response: &Response<axum::body::Body>,
					 _latency: Duration,
					 span: &tracing::Span| {
						// log with error level if the http response is an error
						let status = response.status();
						if status.is_server_error() || status.is_client_error() {
							span.in_scope(|| {
								tracing::error!(
									status = %status,
								);
							});
						} else {
							span.in_scope(|| {
								tracing::info!("");
							});
						};
					},
				),
		);

	let addr = SocketAddr::from(([0, 0, 0, 0], port));

	let local_ip_address = local_ip_address::local_ip()
		.map(|ip| ip.to_string())
		.unwrap_or("localhost".into());

	println!(
		"Serving source list at http://{}:{}",
		local_ip_address, port
	);
	println!(
		"Add http://{}:{}/index.min.json as a source list in Buny",
		local_ip_address, port
	);
	println!("Hit CTRL-C to stop the server.");

	// start http server
	axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
		.await
		.map_err(anyhow::Error::from)
}
