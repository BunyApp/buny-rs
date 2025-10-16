#![no_std]
use buny::{
	alloc::{vec, String, Vec},
	imports::{defaults::defaults_get, net::Request},
	prelude::*,
	AlternateCoverProvider, Chapter, CheckFilter, ContentRating, DeepLinkHandler, DeepLinkResult,
	DynamicFilters, DynamicListings, DynamicSettings, Filter, FilterValue, Home, HomeComponent,
	HomeLayout, Listing, ListingProvider, Novel, NovelPageResult, NovelStatus,
	MigrationHandler, MultiSelectFilter, NotificationHandler, Content, ContentBlock,
    RangeFilter, Result, SelectFilter, Setting, SortFilter, Source,
	TextFilter, ToggleSetting
};

const PAGE_SIZE: i32 = 20;

// to create a source, you need a struct that implements the Source trait
// the struct can contain properties that are initialized with the new() method
struct ExampleSource;

impl Source for ExampleSource {
	// this method is called once when the source is initialized
	// perform any necessary setup here
	fn new() -> Self {
		Self
	}

	// this method will be called first without a query when the search page is opened,
	// then when a search query is entered or filters are changed
	fn get_search_novel_list(
		&self,
		query: Option<String>,
		page: i32,
		_filters: Vec<FilterValue>,
	) -> Result<NovelPageResult> {
		let mut entries: Vec<Novel> = Vec::new();
		let start = (page - 1) * PAGE_SIZE + 1;
		for i in start..start + PAGE_SIZE {
			let title = format!("Novel {i}");
			if let Some(query) = query.as_ref() {
				if !title.contains(query) {
					continue;
				}
			}
			entries.push(Novel {
				key: format!("{i}"),
				title,
				cover: Some(String::from("https://aidoku.app/images/icon.png")),
				authors: Some(vec![String::from("Author")]),
				..Default::default()
			})
		}
		Ok(NovelPageResult {
			entries,
			has_next_page: start < 40,
		})
	}

	// this method will be called when a novel page is opened
	fn get_novel_update(
		&self,
		mut novel: Novel,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Novel> {
		if needs_details {
			novel.authors = Some(vec![String::from("Author")]);
			novel.description = ExampleSource::get_latest_buny_version();
			novel.status = NovelStatus::Ongoing;
			novel.content_rating = ContentRating::Safe;
			novel.tags = Some(vec![String::from("Tag 1"), String::from("Tag 2")]);
			novel.url = Some(String::from("https://buny.app"));
		}
		if needs_chapters {
			novel.chapters = Some(vec![
				Chapter {
					key: String::from("8"),
					chapter_number: Some(8.0),
					..Default::default()
				},
				Chapter {
					key: String::from("7"),
					chapter_number: Some(7.0),
					title: Some(String::from("Title")),
					..Default::default()
				},
				Chapter {
					key: String::from("6"),
					chapter_number: Some(6.0),
					title: Some(String::from("Title")),
					date_uploaded: Some(1692318525),
					..Default::default()
				},
				Chapter {
					key: String::from("5"),
					chapter_number: Some(5.0),
					..Default::default()
				},
				Chapter {
					key: String::from("4"),
					chapter_number: Some(4.0),
					..Default::default()
				},
				Chapter {
					key: String::from("3"),
					chapter_number: Some(3.0),
					..Default::default()
				},
				Chapter {
					key: String::from("2"),
					chapter_number: Some(2.0),
					..Default::default()
				},
				Chapter {
					key: String::from("1"),
					chapter_number: Some(1.0),
					..Default::default()
				},
			]);
		}
		Ok(novel)
	}

	fn get_content_list(&self, _novel: Novel, _chapter: Chapter) -> Result<Vec<Content>> {
		Ok(vec![
			Content {
				content: ContentBlock::block_quote("https://aidoku.app/images/icon.png"),
				..Default::default()
			},
			Content {
				content: ContentBlock::paragraph(
					"# Title\n\nThis is some description\n\n## Section\n\nThis is a section.",
                    None
				),
				..Default::default()
			},
		])
	}
}

impl ExampleSource {
	// gets the latest version of buny from the github releases page
	fn get_latest_buny_version() -> Option<String> {
		Request::get("https://github.com/aidoku/aidoku/releases")
			.ok()?
			.html()
			.ok()?
			.select_first(".repository-content .Box a")?
			.text()
	}
}

// if your source provides any listings (static, dynamic, or in home components), this trait must be implemented
// this should probably be most sources
impl ListingProvider for ExampleSource {
	// this method will be called when a listing or a home section with an associated listing is opened
	fn get_novel_list(&self, listing: Listing, _page: i32) -> Result<NovelPageResult> {
		if listing.id == "test" {
			bail!("Not supported");
		}
		Ok(NovelPageResult {
			entries: vec![Novel {
				key: String::from("1"),
				title: String::from("Novel 1"),
				cover: Some(String::from("https://buny.app/images/icon.png")),
				..Default::default()
			}],
			has_next_page: false,
		})
	}
}

// use the home trait to implement a home page for a source
// where possible, try to replicate the associated web page's layout
impl Home for ExampleSource {
	fn get_home(&self) -> Result<HomeLayout> {
		let entries = self.get_search_novel_list(None, 1, Vec::new())?.entries;
		let chapter = Chapter {
			key: String::from("1"),
			chapter_number: Some(1.0),
			title: Some(String::from("Chapter")),
			date_uploaded: Some(1692318525),
			..Default::default()
		};
		Ok(HomeLayout {
			components: vec![
				HomeComponent {
					title: Some(String::from("Horizontal Scroller")),
					subtitle: None,
					value: buny::HomeComponentValue::Scroller {
						entries: entries.clone(),
						auto_scroll_interval: Some(10.0),
                        listing: None,
                        size: 200,
					},
				},
               
			],
		})
	}
}


// if your source changes filters frequently or only has some filters available conditionally, use the DynamicFilters trait
// where possible, static filters are preferred
impl DynamicFilters for ExampleSource {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
		Ok(vec![
			TextFilter {
				id: "text".into(),
				title: Some("Text".into()),
				placeholder: Some("Search".into()),
				..Default::default()
			}
			.into(),
			SortFilter {
				id: "sort".into(),
				title: Some("Sort".into()),
				can_ascend: true,
				options: vec!["Popular".into(), "Recent".into()],
				..Default::default()
			}
			.into(),
			CheckFilter {
				id: "check".into(),
				title: Some("Check".into()),
				can_exclude: true,
				..Default::default()
			}
			.into(),
			SelectFilter {
				id: "select".into(),
				title: Some("Select".into()),
				uses_tag_style: true,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			MultiSelectFilter {
				id: "mselect".into(),
				title: Some("Multi-Select".into()),
				can_exclude: true,
				uses_tag_style: false,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			Filter::note("Testing note"),
			RangeFilter {
				id: "range".into(),
				title: Some("Range".into()),
				min: Some(0.0),
				max: Some(100.0),
				decimal: true,
				..Default::default()
			}
			.into(),
		])
	}
}

// if you need to serve settings dynamically, use the DynamicSettings trait
// again, this shouldn't be used for static settings
impl DynamicSettings for ExampleSource {
	fn get_dynamic_settings(&self) -> Result<Vec<Setting>> {
		let toggle_value = defaults_get::<bool>("setting");
		let mut settings = vec![ToggleSetting {
			key: "setting".into(),
			title: "Toggle".into(),
			notification: Some("test".into()),
			refreshes: Some(vec!["settings".into()]),
			..Default::default()
		}
		.into()];
		if let Some(value) = toggle_value {
			if value {
				settings.push(
					ToggleSetting {
						key: "setting2".into(),
						title: "Toggle 2".into(),
						..Default::default()
					}
					.into(),
				);
			}
		}
		Ok(settings)
	}
}

// if you need to serve listings dynamically, use the DynamicListings trait
// again, this shouldn't be used for static listings
// for example, you could fetch listings from an API, or show one if a certain setting is enabled
impl DynamicListings for ExampleSource {
	fn get_dynamic_listings(&self) -> Result<Vec<Listing>> {
		Ok(vec![Listing {
			id: String::from("listing"),
			name: String::from("Listing"),
			kind: buny::ListingKind::List,
		}])
	}
}

// if you need to perform any actions when settings change, use the NotificationHandler trait
// for example, you could update different defaults values
impl NotificationHandler for ExampleSource {
	fn handle_notification(&self, key: String) {
		println!("Notification: {key}");
	}
}

// if your source supports displaying multiple covers for a title, use the AlternateCoverProvider trait
impl AlternateCoverProvider for ExampleSource {
	fn get_alternate_covers(&self, _novel: Novel) -> Result<Vec<String>> {
		Ok(vec!["https://buny.app/images/icon.png".into()])
	}
}

// it's recommended for all sources to implement the DeepLinkHandler trait
// the url that is passed in will have the base of any of the source's urls
// the source should determine if the url is a link to a novel, a chapter, or a listing page,
// then return the appropriate DeepLinkResult to handle it.
impl DeepLinkHandler for ExampleSource {
	fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
		Ok(Some(DeepLinkResult::Novel {
			key: String::from("novel_key"),
		}))
	}
}

// the register_source! macro generates the necessary wasm functions for buny
register_source!(
	ExampleSource,
	// after the name of the source struct, list all the extra traits it implements
	ListingProvider,
	Home,
	DynamicFilters,
	DynamicSettings,
	DynamicListings,
	NotificationHandler,
	AlternateCoverProvider,
	DeepLinkHandler
);

// you can also implement tests via our custom test runner!
#[cfg(test)]
mod test {
	use super::*;
	use buny_test::buny_test;

	// all tests need to be annotated with the #[buny_test] attribute instead of #[test]
	#[buny_test]
	fn test_request() {
		let version = ExampleSource::get_latest_buny_version();
		println!("{:?}", version); // if the test fails (or you pass --nocapture), you can see this in the log,
		assert!(version.is_some());
		assert!(version.unwrap().chars().next().unwrap() == 'v');
	}

	#[buny_test]
	fn test_js_execution() {
		// most buny imports you'd want to use should also work
		use buny::imports::js::JsContext;
		let context = JsContext::new();
		let result = context.eval("1 + 2");
		assert_eq!(result, Ok(String::from("3")));
	}
}
