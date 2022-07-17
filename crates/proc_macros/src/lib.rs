use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DeriveInput, Ident, ItemFn, Type};

#[proc_macro_attribute]
pub fn initialize(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;

    quote! {
        #func

        #[no_mangle]
        #[export_name = "initialize"]
        pub unsafe extern "C" fn __wasm_initialize() {
            #func_name()
        }
    }
    .into()
}

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
                                value: filter_ref.get("value").clone(),
                                object: filter_ref.clone(),
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
    }
    .into()
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
                Ok(name) => name.read(),
                Err(_) => return -1,
            };
            let listing = Listing { name: name };
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
            let obj = aidoku::std::ObjectRef(aidoku::std::ValueRef::new(chapter_rid));
            let id = match obj.get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return -1,
            };
            let manga_id = match obj.get("mangaId").as_string() {
                Ok(id) => id.read(),
                Err(_) => return -1,
            };
            let resp: Result<Vec<Page>> = #func_name(manga_id, id);
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
pub fn modify_image_request(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "modify_image_request"]
        pub unsafe extern "C" fn __wasm_modify_image_request(request_rid: i32) {
            let request = aidoku::std::net::Request(request_rid);
            #func_name(request);
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn handle_url(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "handle_url"]
        pub unsafe extern "C" fn __wasm_handle_url(url_rid: i32) -> i32 {
            let url = match aidoku::std::ValueRef::new(url_rid).as_string() {
                Ok(url) => url.read(),
                Err(_) => return -1,
            };
            let resp: Result<DeepLink> = #func_name(url);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => -1,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn handle_notification(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "handle_notification"]
        pub unsafe extern "C" fn __wasm_handle_notification(notification_rid: i32) {
            let notification = match aidoku::std::ValueRef::new(notification_rid).as_string() {
                Ok(notification) => notification.read(),
                Err(_) => return,
            };
            #func_name(notification);
        }
    }
    .into()
}

#[proc_macro_derive(Deserializable, attributes(alias))]
pub fn from_objectref(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("impl must be a struct"),
    };

    // convert all the field names into strings
    // if there is an attribute use that instead of the field name
    let mut keys = Vec::with_capacity(fields.len());
    for field in fields.clone() {
        if field.attrs.is_empty() {
            keys.push(field.ident.unwrap().to_string());
        } else {
            for attr in field.attrs {
                if attr.path.is_ident("alias") {
                    let meta: syn::Meta = attr.parse_meta().unwrap();
                    if let syn::Meta::NameValue(name_value) = meta {
                        if let syn::Lit::Str(lit) = name_value.lit {
                            keys.push(lit.value());
                        } else {
                            panic!("alias must be a string");
                        }
                    } else {
                        panic!("expected name-value pair: #[alias = \"...\"]");
                    }
                    break;
                }
            }
        }
    }

    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    fn get_base_typecall<T: AsRef<str>>(key: T, typcall: T) -> quote::__private::TokenStream {
        let key = key.as_ref();
        let typcall = typcall.as_ref();
        match typcall {
            "stringref" => quote! {obj.get(#key).as_string()},
            "objectref" => quote! {obj.get(#key).as_object()},
            "arrayref" => quote! {obj.get(#key).as_array()},
            "valueref" => quote! {obj.get(#key)},
            "i8" | "i16" | "i32" | "i64" => quote! {obj.get(#key).as_int()},
            "f32" | "f64" => quote! {obj.get(#key).as_float()},
            "bool" => quote! {obj.get(#key).as_bool()},
            &_ => panic!("unimplemented type {typcall}"),
        }
    }

    let typecalls = keys
        .iter()
        .zip(fields.iter())
        .map(|(key, field)| match field.ty.clone() {
            Type::Path(typepath) => {
                // TODO: options and results
                // TODO: vecs
                // TODO: genericized numerics

                // get the type of the specified field, lowercase
                let typ = typepath.path.segments.last().unwrap();
                let typiden = typ.ident.to_string().to_lowercase();
                match typiden.as_str() {
                    "string" => quote! {obj.get(#key).as_string()?.read()},
                    "option" => {
                        // extract the type of the option
                        if let syn::PathArguments::AngleBracketed(typargs) = &typ.arguments {
                            if let syn::GenericArgument::Type(last) = typargs.args.last().unwrap() {
                                if let Type::Path(typepath) = last {
                                    let typ = typepath.path.segments.last().unwrap();
                                    let typiden = typ.ident.to_string().to_lowercase();
                                    match typiden.as_str() {
                                        "string" => quote! {
                                            match obj.get(#key).as_string() {
                                                Ok(s) => Some(s.read()),
                                                Err(_) => None,
                                            }
                                        },
                                        &_ => { 
                                            let call = get_base_typecall(key, &typiden);
                                            match typiden.as_str() {
                                                "i8" | "i16" | "i32" | "f32" | "f64" => quote! {
                                                    match #call {
                                                        Ok(s) => match s.try_into() {
                                                            Ok(s) => Some(s),
                                                            Err(_) => None,
                                                        }
                                                        Err(_) => None,
                                                    }
                                                },
                                                "stringref" | "arrayref" | "valueref" | "objectref" => quote! {
                                                    match #call {
                                                        Ok(s) => Some(s.clone()),
                                                        Err(_) => None,
                                                    }
                                                },
                                                _ => quote! {
                                                    match #call {
                                                        Ok(s) => Some(s),
                                                        Err(_) => None,
                                                    }
                                                },
                                            }
                                        },
                                    }
                                } else {
                                    panic!("expected type path");
                                }
                            } else {
                                panic!("expected type argument to option");
                            }
                        } else {
                            panic!("expected angle brackets after option");
                        }
                    },
                    &_ => { 
                        let call = get_base_typecall(key, &typiden);
                        let extra_typcall = match typiden.as_str() {
                            "i8" | "i16" | "i32" => quote! {.try_into().unwrap_or(0)},
                            "f32" | "f64" => quote! {.try_into().unwrap_or(0.0)},
                            "stringref" | "arrayref" | "objectref" => quote! {?.clone()},
                            "valueref" => quote! {.clone()},
                            &_ => quote! {},
                        };
                        quote! {#call #extra_typcall}
                    },
                }
            }
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics core::convert::TryFrom<aidoku::std::ObjectRef> for #name #ty_generics #where_clause {
            type Error = aidoku::error::AidokuError;

            fn try_from(obj: aidoku::std::ObjectRef) -> aidoku::error::Result<Self> {
                Ok(Self {
                    #(#idents: #typecalls,)*
                })
            }
        }
    }
    .into()
}

