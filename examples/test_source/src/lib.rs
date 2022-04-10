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
pub unsafe extern "C" fn get_manga_list(_filters_rid: i32, page: i32) -> i32 {
    // let mut filters: aidoku::std::Vec<aidoku::Filter> = Vec::new();
    // if filters_rid > -1 {
    //     let filters_ref = aidoku::std::ValueRef::new(filters_rid);
    //     if let Ok(arr) = filters_ref.as_array() {
    //         for item in arr {
    //             let filter = aidoku::Filter {
    //                 kind: aidoku::FilterType::Text,
    //                 name: "",
    //                 value_ptr: 0 as *const i32,
    //                 value2_ptr: 0 as *const i32,
    //             };
    //             filter.value_ptr = filter.create();
    //             filters.push(filter);
    //         }
    //     }
    // }
    let resp: aidoku::error::Result<aidoku::MangaPageResult> = get_manga_list_rs(Vec::new(), page);
    match resp {
        Ok(resp) => resp.create(),
        Err(_) => -2,
    }
}

// #[manga_list_request]
fn get_manga_list_rs(_: Vec<Filter>, _: i32) -> Result<MangaPageResult> {
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
