use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemFn};

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

/// Retrieves the default list of novels for the source, which may be sorted
/// alphabetically or based on the most relevant default page.
#[proc_macro_attribute]
pub fn get_novel_list(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        /// test
        #[no_mangle]
        #[export_name = "get_novel_list"]
        pub unsafe extern "C" fn __wasm_get_novel_list(page: i32) -> i32 {
            let mut filters: Vec<Filter> = Vec::new();
            let resp: buny::error::Result<buny::NovelPageResult> = #func_name(page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => i32::from(-401),
            }
        }
    }
    .into()
}

/// Retrieves categorized novel listings such as "Latest", "Popular", or "Updated".
#[proc_macro_attribute]
pub fn get_novel_listing(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_novel_listing"]
        pub unsafe extern "C" fn __wasm_get_novel_listing(listing_rid: i32, page: i32) -> i32 {
            let name = match buny::std::ObjectRef(buny::std::ValueRef::new(listing_rid)).get("name").as_string() {
                Ok(name) => name.read(),
                Err(_) => return i32::from(-402),
            };
            let listing = Listing { name: name };
            let resp: Result<NovelPageResult> = #func_name(listing, page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => i32::from(-401),
            }
        }
    }
    .into()
}

/// Searches for novels based on user-provided filters and keywords.
#[proc_macro_attribute]
pub fn get_search_result(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_search_result"]
        pub unsafe extern "C" fn __wasm_get_search_result(filters_rid: i32, page: i32) -> i32 {
            let mut filters: Vec<Filter> = Vec::new();
            if filters_rid > -1 {
                let filters_ref = buny::std::ValueRef::new(filters_rid);
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
                                kind: buny::FilterType::from(fiter_type as i32),
                                name: name.read(),
                                value: filter_ref.get("value").clone(),
                                object: filter_ref.clone(),
                            };
                            filters.push(filter);
                        }
                    }
                }
            }
            let resp: buny::error::Result<buny::NovelPageResult> = #func_name(filters, page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => i32::from(-401),
            }
        }
    }
    .into()
}

/// Fetches detailed information about a novel, including its author, cover, and title.
#[proc_macro_attribute]
pub fn get_novel_details(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_novel_details"]
        pub unsafe extern "C" fn __wasm_get_novel_details(novel_rid: i32) -> i32 {
            let id = match buny::std::ObjectRef(buny::std::ValueRef::new(novel_rid)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return i32::from(-402),
            };
            let resp: Result<Novel> = #func_name(id);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) => i32::from(-401),
            }
        }
    }
    .into()
}

/// Get all the reviews for the novel if available.
#[proc_macro_attribute]
pub fn get_novel_reviews(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_novel_reviews"]
        pub unsafe extern "C" fn __wasm_get_novel_reviews(novel_rid: i32) -> i32 {
            let id = match buny::std::ObjectRef(buny::std::ValueRef::new(novel_rid)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return i32::from(-402),
            };
            let resp: Result<Vec<Review>> = #func_name(id);
            match resp {
                Ok(resp) => {
                    let mut arr = buny::std::ArrayRef::new();
                    for item in resp {
                        let rid = item.create();
                        arr.insert(buny::std::ValueRef::new(rid));
                    }
                    let rid = arr.0.0;
                    core::mem::forget(arr.0);
                    rid
                }
                Err(_) => i32::from(-401),
            }
        }
    }
    .into()
}

/// Retrieves the list of chapters for a given novel.
#[proc_macro_attribute]
pub fn get_chapter_list(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func

        #[no_mangle]
        #[export_name = "get_chapter_list"]
        pub unsafe extern "C" fn __wasm_get_chapter_list(novel_id: i32, page: i32) -> i32 {
            let id = match buny::std::ObjectRef(buny::std::ValueRef::new(novel_id)).get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return i32::from(-402),
            };
            let resp: buny::error::Result<ChapterPageResult> = #func_name(id, page);
            match resp {
                Ok(resp) => resp.create(),
                Err(_) =>i32::from(-401),
            }
        }
    }
    .into()
}

/// Fetches the content of a specific chapter as an array of paragraphs.
#[proc_macro_attribute]
pub fn get_chapter_content(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse(input).expect("expected the attribute to be used on a function");
    let func_name = &func.sig.ident;
    quote! {
        #func
        
        #[no_mangle]
        #[export_name = "get_chapter_content"]
        pub unsafe extern "C" fn __wasm_get_chapter_content(chapter_rid: i32) -> i32 {
            let obj = buny::std::ObjectRef(buny::std::ValueRef::new(chapter_rid));
            let id = match obj.get("id").as_string() {
                Ok(id) => id.read(),
                Err(_) => return i32::from(-403),
            };
            let novel_id = match obj.get("novelId").as_string() {
                Ok(id) => id.read(),
                Err(_) => return i32::from(-402),
            };
            let resp: Result<Vec<ChapterParagraph>> = #func_name(novel_id, id);
            match resp {
                Ok(resp) => {
                    let mut arr = buny::std::ArrayRef::new();
                    for item in resp {
                        let rid = item.create();
                        arr.insert(buny::std::ValueRef::new(rid));
                    }
                    let rid = arr.0.0;
                    core::mem::forget(arr.0);
                    rid
                }
                Err(_) => i32::from(-401),
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
            let request = buny::std::net::Request(request_rid, false);
            #func_name(request);

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
            let notification = match buny::std::ValueRef::new(notification_rid).as_string() {
                Ok(notification) => notification.read(),
                Err(_) => return,
            };
            #func_name(notification);
        }
    }
    .into()
}
