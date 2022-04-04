use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemFn};

#[proc_macro_attribute]
pub fn manga_list_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        #[no_mangle]
        pub unsafe extern "C" fn manga_list_request(filters: *const i32, page: i32) -> i32 {
            let resp = #func_name(alloc::vec::Vec::new(), page);
            match resp {
                Ok(resp) => resp,
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn manga_listing_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        #[no_mangle]
        pub unsafe extern "C" fn manga_listing_request(filters: *const i32, page: i32) -> i32 {
            let resp = #func_name();
            match resp {
                Ok(resp) => resp,
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn manga_details_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        #[no_mangle]
        pub unsafe extern "C" fn manga_details_request(filters: *const i32, page: i32) -> i32 {
            let resp = #func_name();
            match resp {
                Ok(resp) => resp,
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn chapter_list_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        #[no_mangle]
        pub unsafe extern "C" fn chapter_list_request(filters: *const i32, page: i32) -> i32 {
            let resp = #func_name();
            match resp {
                Ok(resp) => resp,
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn page_list_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        #[no_mangle]
        pub unsafe extern "C" fn page_list_request(chapter: *const i32) -> i32 {
            let resp = #func_name();
            match resp {
                Ok(resp) => resp,
                Err(_) => -1,
            }
        }
    }
    .into()
}
