#![no_std]
use aidoku::{
    error::Result,
    std::String,
    // prelude::*,
    std::Vec,
    Filter,
    Listing,
    Manga,
    MangaPageResult,
    Chapter,
    Page,
};

#[no_mangle]
#[export_name = "get_manga_list"]
pub unsafe extern "C" fn __wasm_get_manga_list(filters_rid: i32, page: i32) -> i32 {
    let mut filters: Vec<Filter> = Vec::new();
    if filters_rid > -1 {
        let filters_ref = aidoku::std::ValueRef::new(filters_rid);
        if let Ok(arr) = filters_ref.as_array() {
            for item in arr {
                let filter_ref = match item.as_object() {
                    Ok(filter_ref) => filter_ref,
                    Err(_) => continue,
                };
                let name = match filter_ref.get("name").as_string() {
                    Ok(name) => name,
                    Err(_) => continue,
                };
                let filter = Filter {
                    kind: aidoku::FilterType::from_i64(filter_ref.get("type").as_int().unwrap_or(0)),
                    name: name.read(),
                    value: filter_ref.get("value")
                };
                filters.push(filter);
            }
        }
    }
    let resp: aidoku::error::Result<aidoku::MangaPageResult> = get_manga_list(filters, page);
    match resp {
        Ok(resp) => resp.create(),
        Err(_) => -1,
    }
}

#[no_mangle]
#[export_name = "get_manga_listing"]
pub unsafe extern "C" fn __wasm_get_manga_listing(listing_rid: i32, page: i32) -> i32 {
    let name = match aidoku::std::ObjectRef(listing_rid).get("name").as_string() {
        Ok(name) => name,
        Err(_) => return -1,
    };
    let listing = Listing {
        name: name.read()
    };
    let resp: Result<MangaPageResult> = get_manga_listing(listing, page);
    match resp {
        Ok(resp) => resp.create(),
        Err(_) => -1,
    }
}

#[no_mangle]
#[export_name = "get_manga_details"]
pub unsafe extern "C" fn __wasm_get_manga_details(manga_rid: i32) -> i32 {
    let id = match aidoku::std::ObjectRef(manga_rid).get("id").as_string() {
        Ok(id) => id,
        Err(_) => return -1,
    };
    let resp: Result<Manga> = get_manga_details(id.read());
    match resp {
        Ok(resp) => resp.create(),
        Err(_) => -1,
    }
}

#[no_mangle]
#[export_name = "get_chapter_list"]
pub unsafe extern "C" fn __wasm_get_chapter_list(manga_rid: i32) -> i32 {
    let id = match aidoku::std::ObjectRef(manga_rid).get("id").as_string() {
        Ok(id) => id,
        Err(_) => return -1,
    };
    let resp: Result<Vec<Chapter>> = get_chapter_list(id.read());
    match resp {
        Ok(resp) => {
            let mut arr = aidoku::std::ArrayRef::new();
            for item in resp {
                let rid = item.create();
                arr.insert(aidoku::std::ValueRef::new(rid));
            }
            arr.0
        },
        Err(_) => -1,
    }
}

#[no_mangle]
#[export_name = "get_page_list"]
pub unsafe extern "C" fn __wasm_get_page_list(chapter_rid: i32) -> i32 {
    let id = match aidoku::std::ObjectRef(chapter_rid).get("id").as_string() {
        Ok(id) => id,
        Err(_) => return -1,
    };
    let resp: Result<Vec<Page>> = get_page_list(id.read());
    match resp {
        Ok(resp) => {
            let mut arr = aidoku::std::ArrayRef::new();
            for item in resp {
                let rid = item.create();
                arr.insert(aidoku::std::ValueRef::new(rid));
            }
            arr.0
        },
        Err(_) => -1,
    }
}

// #[manga_list_request]
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
        manga: manga,
        has_more: false,
    })
}

fn get_manga_listing<'a>(_: Listing, _: i32) -> Result<MangaPageResult<'a>> {
    Ok(MangaPageResult {
        manga: Vec::new(),
        has_more: false,
    })
}

fn get_manga_details<'a>(_: String) -> Result<Manga<'a>> {
    Err(aidoku::error::AidokuError { reason: aidoku::error::AidokuErrorKind::Unimplemented })
}

fn get_chapter_list<'a>(_: String) -> Result<Vec<Chapter<'a>>> {
    Err(aidoku::error::AidokuError { reason: aidoku::error::AidokuErrorKind::Unimplemented })
}

fn get_page_list<'a>(_: String) -> Result<Vec<Page<'a>>> {
    Err(aidoku::error::AidokuError { reason: aidoku::error::AidokuErrorKind::Unimplemented })
}
