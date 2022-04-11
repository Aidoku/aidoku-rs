# Aidoku Rust Source
**NOTE:** This isn't compatible with the current version of Aidoku (yet), it's just meant for drafting out the new format.

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
fn get_manga_list<'a>(_: Vec<Filter>, _: i32) -> Result<MangaPageResult<'a>> {
    todo!()
}

#[get_manga_listing]
fn get_manga_listing<'a>(_: Listing, _: i32) -> Result<MangaPageResult<'a>> {
    todo!()
}

#[get_manga_details]
fn get_manga_details<'a>(_: String) -> Result<Manga<'a>> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list<'a>(_: String) -> Result<Vec<Chapter<'a>>> {
    todo!()
}

#[get_page_list]
fn get_page_list<'a>(_: String) -> Result<Vec<Page<'a>>> {
    todo!()
}
```

And now you're ready to start making your source!
