#![no_std]
use aidoku::{
    error::Result,
    // prelude::*,
    std::Vec,
    Filter,
    Manga,
    MangaPageResult,
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
        Err(_) => -2,
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
