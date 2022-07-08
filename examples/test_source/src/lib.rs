#![no_std]
use aidoku::{
    error::Result,
    prelude::*,
    std::{net::Request, Deserializable, String, Vec},
    Chapter, DeepLink, Filter, Listing, Manga, MangaPageResult, Page,
};

#[derive(Default, Deserializable)]
struct MangaDexChapter {
    id: String,
    timestamp: i64,
    #[alias = "externalUrl"]
    external_url: aidoku::std::ValueRef,
}

#[initialize]
fn initialize() {
    // Place any code that is supposed to run once when the source starts here.
    // This include initializing any variables, setting the rate limit, etc.
    todo!()
}

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, _page: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_listing]
fn get_manga_listing(_listing: Listing, _page: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_details]
fn get_manga_details(_id: String) -> Result<Manga> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_id: String) -> Result<Vec<Chapter>> {
    todo!()
}

#[get_page_list]
fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>> {
    todo!()
}

#[modify_image_request]
fn modify_image_request(_request: Request) {
    todo!()
}

#[handle_url]
fn handle_url(_url: String) -> Result<DeepLink> {
    todo!()
}

#[handle_notification]
fn handle_notification(_notification: String) {
    todo!()
}
