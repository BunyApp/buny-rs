# Buny Rust Source API

## Setup
To make a new source:
```shell
cargo new <source_name>
```

Then in the Cargo.toml add:
```toml
[lib]
crate-type = ["cdylib"]

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

Now, in your src/main.rs file, simply add:
```rs
#![no_std]
use buny::{
    error::Result,
    prelude::*,
    std::{String, Vec},
    Chapter, Filter, Listing, novel, PageResult, Page,
};

#[get_novel_list]
fn get_novel_list(_: Vec<Filter>, _: i32) -> Result<PageResult> {
    todo!()
}

#[get_novel_listing]
fn get_novel_listing(_: Listing, _: i32) -> Result<PageResult> {
    todo!()
}

#[get_novel_details]
fn get_novel_details(_: String) -> Result<novel> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_: String) -> Result<Vec<Chapter>> {
    todo!()
}

#[get_page_list]
fn get_page_list(_: String, _: String) -> Result<Vec<Page>> {
    todo!()
}
```

And now you're ready to start making your source!
