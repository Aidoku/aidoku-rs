#![no_std]
use aidoku::{
    error::Result, prelude::*, std::String, std::Vec, Chapter, Filter, Listing, Manga,
    MangaPageResult, Page,
};

#[get_manga_list]
fn get_manga_list<'a>(_: Vec<Filter>, _: i32) -> Result<MangaPageResult<'a>> {
    let mut manga: Vec<Manga> = Vec::new();
    manga.push(Manga {
        id: "1",
        cover: "https://skitty.xyz/icon.png",
        title: "Title",
        author: "Author",
        artist: "",
        description: "Description",
        url: "",
        categories: Vec::new(),
        status: aidoku::MangaStatus::Unknown,
        nsfw: aidoku::MangaContentRating::Safe,
        viewer: aidoku::MangaViewer::Default,
    });
    Ok(MangaPageResult {
        manga,
        has_more: false,
    })
}

#[get_manga_listing]
fn get_manga_listing<'a>(_: Listing, _: i32) -> Result<MangaPageResult<'a>> {
    Ok(MangaPageResult {
        manga: Vec::new(),
        has_more: false,
    })
}

#[get_manga_details]
fn get_manga_details<'a>(_: String) -> Result<Manga<'a>> {
    Err(aidoku::error::AidokuError {
        reason: aidoku::error::AidokuErrorKind::Unimplemented,
    })
}

#[get_chapter_list]
fn get_chapter_list<'a>(_: String) -> Result<Vec<Chapter<'a>>> {
    Err(aidoku::error::AidokuError {
        reason: aidoku::error::AidokuErrorKind::Unimplemented,
    })
}

#[get_page_list]
fn get_page_list<'a>(_: String) -> Result<Vec<Page<'a>>> {
    Err(aidoku::error::AidokuError {
        reason: aidoku::error::AidokuErrorKind::Unimplemented,
    })
}
