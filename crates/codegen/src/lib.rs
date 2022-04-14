use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemFn};

#[proc_macro_attribute]
pub fn get_manga_list(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

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
                        if let Ok(fiter_type) = filter_ref.get("type").as_int() {
                            let filter = Filter {
                                kind: aidoku::FilterType::from(fiter_type as i32),
                                name: name.read(),
                                value: filter_ref.get("value")
                            };
                            filters.push(filter);
                        }
                    }
                }
            }
            let resp: aidoku::error::Result<aidoku::MangaPageResult> = #func_name(filters, page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => -1,
            }
        }
    }.into()
}

#[proc_macro_attribute]
pub fn get_manga_listing(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_manga_listing"]
        pub unsafe extern "C" fn __wasm_get_manga_listing(listing_rid: i32, page: i32) -> i32 {
            let name = match aidoku::std::ObjectRef(aidoku::std::ValueRef::new(listing_rid)).get("name").as_string() {
                Ok(name) => name,
                Err(_) => return -1,
            };
            let listing = Listing { name: name.read() };
            let resp: Result<MangaPageResult> = #func_name(listing, page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn get_manga_details(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_manga_details"]
        pub unsafe extern "C" fn __wasm_get_manga_details(manga_rid: i32) -> i32 {
            let id = match aidoku::std::ObjectRef(aidoku::std::ValueRef::new(manga_rid)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return -1,
            };
            let resp: Result<Manga> = #func_name(id);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn get_chapter_list(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_chapter_list"]
        pub unsafe extern "C" fn __wasm_get_chapter_list(manga_rid: i32) -> i32 {
            let id = match aidoku::std::ObjectRef(aidoku::std::ValueRef::new(manga_rid)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return -1,
            };
            let resp: Result<Vec<Chapter>> = #func_name(id);
            match resp {
                Ok(resp) => {
                    let mut arr = aidoku::std::ArrayRef::new();
                    for item in resp {
                        let rid = item.create();
                        arr.insert(aidoku::std::ValueRef::new(rid));
                    }
                    let rid = arr.0.0;
                    core::mem::forget(arr.0);
                    rid
                }
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn get_page_list(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_page_list"]
        pub unsafe extern "C" fn __wasm_get_page_list(chapter_rid: i32) -> i32 {
            let id = match aidoku::std::ObjectRef(aidoku::std::ValueRef::new(chapter_rid)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return -1,
            };
            let resp: Result<Vec<Page>> = #func_name(id);
            match resp {
                Ok(resp) => {
                    let mut arr = aidoku::std::ArrayRef::new();
                    for item in resp {
                        let rid = item.create();
                        arr.insert(aidoku::std::ValueRef::new(rid));
                    }
                    let rid = arr.0.0;
                    core::mem::forget(arr.0);
                    rid
                }
                Err(_) => -1,
            }
        }
    }
    .into()
}
