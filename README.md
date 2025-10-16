# Buny Rust Source API

<<<<<<< Updated upstream
## Setup
To make a new source:
```shell
cargo new <source_name>
```

Then in the Cargo.toml add:
```toml
[lib]
crate-type = ["cdylib"]
=======
This repo contains the following crates:
- [buny](crates/lib): A wrapper for Aidoku source libraries.
- [buny-cli](crates/cli): A command-line utility for Aidoku source development and testing.
- [buny-test](crates/test-macro): A crate that allows for exposing tests to `buny-test-runner`.
- [buny-test-runner](crates/test-runner): A tool for running tests on Aidoku sources via a custom source runner.

## Aidoku Source Development

To get started with Aidoku source development, you'll need two things: Rust and buny-cli.

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/). For buny-cli, run the following command after installing Rust:

```sh
cargo install --git https://github.com/Aidoku/buny-rs buny-cli
```

Then, create a new source project by running `buny init`.
>>>>>>> Stashed changes

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "s"
strip = true
lto = true

[dependencies]
buny = "0.1.0"
```

Next, make a folder called .cargo and a file called "config.toml". In it, put:
```toml
[build]
target = "wasm32-unknown-unknown"
```

Now, in your src/lib.rs file, simply add:
```rs
#![no_std]
use buny::{
    error::Result,
    prelude::*,
    std::{net::Request, String, Vec},
    ChapterPageResult, ChapterParagraph, Filter, Listing, Novel, NovelPageResult, Review, Chapter
};

#[initialize]
fn initialize() {
    // Place any code that is supposed to run once when the source starts here.
    // This include initializing any variables, setting the rate limit, etc.
    todo!()
}

#[get_novel_list]
fn get_novel_list(_page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_novel_listing]
fn get_novel_listing(_listing: Listing, _page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_search_result]
fn get_search_result(_filters: Vec<Filter>, _page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_novel_details]
fn get_novel_details(_id: String) -> Result<Novel> {
    todo!()
}

#[get_novel_reviews]
fn get_novel_reviews(_id: String) -> Result<Vec<Review>> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_id: String, _page: i32) -> Result<ChapterPageResult> {
    todo!()
}


#[get_chapter_content]
fn get_chapter_content(_novel_id: String, _chapter_id: String) -> Result<Vec<ChapterParagraph>> {
    todo!()
}

#[modify_image_request]
fn modify_image_request(_request: Request) {
    todo!()
}

#[handle_notification]
fn handle_notification(_notification: String) {
    todo!()
}
```

And now you're ready to start making your source!
