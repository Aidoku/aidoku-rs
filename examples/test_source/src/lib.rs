#![no_std]
use aidoku::{
    error::Result,
    // prelude::*,
    std::Vec,
    Filter,
    // Manga,
    MangaPageResult,
};

#[no_mangle]
pub unsafe extern "C" fn manga_list_request(filters_rid: i32, page: i32) -> i32 {
    let mut filters: aidoku::std::Vec<aidoku::Filter> = Vec::new();
    if filters_rid > -1 {
        let filters_ref = aidoku::std::ValueRef::new(filters_rid);
        if let Ok(arr) = filters_ref.as_array() {
            for item in arr {
                let filter = aidoku::Filter {
                    kind: aidoku::FilterType::Text,
                    name: "",
                    value_ptr: 0 as *const i32,
                    value2_ptr: 0 as *const i32,
                };
                filter.value_ptr = filter.create();
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

// #[manga_list_request]
fn get_manga_list(_: Vec<Filter>, _: i32) -> Result<MangaPageResult> {
    Ok(MangaPageResult {
        manga: Vec::new(),
        has_more: false,
    })
}
