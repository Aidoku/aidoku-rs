# Aidoku Rust Source API
**NOTE:** This library currently requires Rust nightly, which can be installed with `rustup default nightly`.

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
aidoku = "0.1.0"
```

Next, make a folder called .cargo and a file called "config". In it, put:
```toml
[build]
target = "wasm32-unknown-unknown"
```

Now, in your src/main.rs file, simply add:
```rs
#![no_std]
use aidoku::{
    error::Result,
    prelude::*,
    std::{String, Vec},
    Chapter, Filter, Listing, Manga, MangaPageResult, Page,
};

#[get_manga_list]
fn get_manga_list(_: Vec<Filter>, _: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_listing]
fn get_manga_listing(_: Listing, _: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_details]
fn get_manga_details(_: String) -> Result<Manga> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_: String) -> Result<Vec<Chapter>> {
    todo!()
}

#[get_page_list]
fn get_page_list(_: String) -> Result<Vec<Page>> {
    todo!()
}
```

And now you're ready to start making your source!
