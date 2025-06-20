/// Prints to Aidoku logs.
///
/// # Examples
///
/// ```ignore
/// println!(); // prints just a newline
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// let local_variable = "some";
/// println!("format {local_variable} arguments");
/// ```
#[macro_export]
macro_rules! println {
	() => {
		::aidoku::alloc::print("");
	};
	($($arg:tt)*) => {
		::aidoku::imports::std::print(&::aidoku::prelude::format!($($arg)*));
	};
}

/// Prints to Aidoku logs if debug assertions are enabled.
#[macro_export]
macro_rules! debug {
	() => {
		#[cfg(debug_assertions)]
		{
			::aidoku::prelude::println!();
		}
	};
	($($arg:tt)*) => {
		#[cfg(debug_assertions)]
		{
			::aidoku::prelude::println!($($arg)*);
		}
	};
}

/// Returns early with an error.
///
/// This macro is equivalent to
/// <code>return Err(AidokuError::message(format!($args\...)))</code>.
///
/// The surrounding function's or closure's return value is required to be
/// <code>Result&lt;_, [aidoku::AidokuError][crate::AidokuError]&gt;</code>.
#[macro_export]
macro_rules! bail {
	($($arg:tt)*) => {
		return ::core::result::Result::Err(::aidoku::AidokuError::message(::aidoku::prelude::format!($($arg)*)));
	};
}

/// Registers a source for use with Aidoku.
///
/// The first argument should be the struct that implements the Source trait, and the
/// following arguments should be all the additional traits that the source implements.
///
/// # Examples
///
/// ```ignore
/// struct TestSource;
///
/// impl Source for TestSource { /* ... */ }
/// impl Home for TestSource { /* ... */ }
///
/// // register TestSource with the extra Home trait
/// register_source!(TestSource, Home);
/// ```
#[macro_export]
macro_rules! register_source {
	($source_type:ty $(, $param:ident)*) => {
		static mut SOURCE: ::core::option::Option<::aidoku::alloc::Box<$source_type>> =
			::core::option::Option::None;

		fn __source() -> &'static mut $source_type {
			unsafe { SOURCE.as_deref_mut().unwrap() }
		}

		fn __handle_result<T: ::aidoku::serde::Serialize>(
			result: ::core::result::Result<T, ::aidoku::imports::error::AidokuError>,
		) -> i32 {
			match &result {
				::core::result::Result::Ok(result) => {
					let mut bytes = ::aidoku::postcard::to_allocvec(result).unwrap();

				 	bytes.splice(0..0, [0,0,0,0,0,0,0,0]);
					let len_bytes = (bytes.len() as i32).to_le_bytes();
					bytes[0..4].copy_from_slice(&len_bytes);
					let cap_bytes = (bytes.capacity() as i32).to_le_bytes();
					bytes[4..8].copy_from_slice(&cap_bytes);

					let ptr = bytes.as_ptr() as i32;
					::core::mem::forget(bytes);
					ptr
				}
				::core::result::Result::Err(err) => __handle_error(err),
			}
		}

		fn __handle_error(error: &::aidoku::imports::error::AidokuError) -> i32 {
			::aidoku::prelude::println!("Error: {:?}", error);
			match error {
				::aidoku::imports::error::AidokuError::Unimplemented => -2,
				::aidoku::imports::error::AidokuError::RequestError(_) => -3,
				::aidoku::imports::error::AidokuError::Message(string) => {
					let mut buffer = (-1 as i32).to_le_bytes().to_vec();

					buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
					buffer.extend_from_slice(&string.as_bytes());

					let cap_bytes = (buffer.capacity() as i32).to_le_bytes();
					buffer[4..8].copy_from_slice(&cap_bytes);
					let len_bytes = (buffer.len() as i32).to_le_bytes();
					buffer[8..12].copy_from_slice(&len_bytes);

					let ptr = buffer.as_ptr() as i32;
					::core::mem::forget(buffer);
					ptr
				}
				_ => -1,
			}
		}

		// once rust supports exporting a wasm start function, we should use that instead
		#[export_name = "start"]
		pub extern "C" fn __start() {
			unsafe {
				SOURCE =
					::core::option::Option::Some(::aidoku::alloc::Box::new(<$source_type>::new()))
			};
		}

		#[no_mangle]
		#[export_name = "free_result"]
		pub unsafe extern "C" fn __wasm_free_result(ptr: i32) {
			::aidoku::imports::std::free_result(ptr);
		}

		#[no_mangle]
		#[export_name = "get_search_manga_list"]
		pub unsafe extern "C" fn __wasm_get_search_manga_list(
			query_descriptor: i32,
			page: i32,
			filters_descriptor: i32,
		) -> i32 {
			let query = ::aidoku::imports::std::read_string(query_descriptor);
			let ::core::result::Result::Ok(filters) =
				::aidoku::imports::std::read::<::aidoku::alloc::Vec<::aidoku::FilterValue>>(filters_descriptor)
			else {
				return -1;
			};

			let result = __source().get_search_manga_list(query, page, filters);
			__handle_result(result)
		}

		#[no_mangle]
		#[export_name = "get_manga_update"]
		pub unsafe extern "C" fn __wasm_get_manga_update(
			manga_descriptor: i32,
			needs_details: bool,
			needs_chapters: bool,
		) -> i32 {
			let ::core::result::Result::Ok(manga) =
				::aidoku::imports::std::read::<::aidoku::Manga>(manga_descriptor)
			else {
				return -1;
			};

			let result = __source().get_manga_update(manga, needs_details, needs_chapters);
			__handle_result(result)
		}

		#[no_mangle]
		#[export_name = "get_page_list"]
		pub unsafe extern "C" fn __wasm_get_page_list(
			manga_descriptor: i32,
			chapter_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(manga) =
				::aidoku::imports::std::read::<::aidoku::Manga>(manga_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(chapter) =
				::aidoku::imports::std::read::<::aidoku::Chapter>(chapter_descriptor)
			else {
				return -2;
			};

			let result = __source()
				.get_page_list(manga, chapter)
				.map(|pages| {
					pages.into_iter()
						.map(|mut page| {
							page.ensure_externally_managed();
							page
						})
						.collect::<::aidoku::alloc::Vec<_>>()
				});
			__handle_result(result)
		}

		$(
			register_source!(@single $param);
		)*
	};

	(@single ListingProvider) => {
		#[no_mangle]
		#[export_name = "get_manga_list"]
		pub unsafe extern "C" fn __wasm_get_manga_list(listing_descriptor: i32, page: i32) -> i32 {
			let ::core::result::Result::Ok(listing) =
				::aidoku::imports::std::read::<::aidoku::Listing>(listing_descriptor)
			else {
				return -1;
			};

			let result = __source().get_manga_list(listing, page);
			__handle_result(result)
		}
	};

	(@single Home) => {
		#[no_mangle]
		#[export_name = "get_home"]
		pub unsafe extern "C" fn __wasm_get_home() -> i32 {
			let result = __source().get_home();
			__handle_result(result)
		}
	};

	(@single DynamicListings) => {
		#[no_mangle]
		#[export_name = "get_listings"]
		pub unsafe extern "C" fn __wasm_get_listings() -> i32 {
			let result = __source().get_dynamic_listings();
			__handle_result(result)
		}
	};

	(@single DynamicFilters) => {
		#[no_mangle]
		#[export_name = "get_filters"]
		pub unsafe extern "C" fn __wasm_get_filters() -> i32 {
			let result = __source().get_dynamic_filters();
			__handle_result(result)
		}
	};

	(@single DynamicSettings) => {
		#[no_mangle]
		#[export_name = "get_settings"]
		pub unsafe extern "C" fn __wasm_get_settings() -> i32 {
			let result = __source().get_dynamic_settings();
			__handle_result(result)
		}
	};

	(@single PageImageProcessor) => {
		#[no_mangle]
		#[export_name = "process_page_image"]
		pub unsafe extern "C" fn __wasm_process_page_image(
			response_descriptor: i32,
			context_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(response) =
				::aidoku::imports::std::read::<::aidoku::ImageResponse>(response_descriptor)
			else {
				return -1;
			};
			let context: ::core::option::Option<::aidoku::PageContext> = if context_descriptor < 0 {
				None
			} else if let ::core::result::Result::Ok(context) =
				::aidoku::imports::std::read::<::aidoku::PageContext>(context_descriptor)
			{
				Some(context)
			} else {
				return -2;
			};

			use ::aidoku::PageImageProcessor;
			let mut result = __source().process_page_image(response, context);
			if let Ok(image_ref) = result.as_mut() {
				image_ref.externally_managed = true;
			}
			__handle_result(result.map(|r| r.rid))
		}
	};

	(@single ImageRequestProvider) => {
		#[no_mangle]
		#[export_name = "get_image_request"]
		pub unsafe extern "C" fn __wasm_get_image_request(
			url_descriptor: i32,
			context_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(url) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(url_descriptor)
			else {
				return -1;
			};
			let context: ::core::option::Option<::aidoku::PageContext> = if context_descriptor < 0 {
				None
			} else if let ::core::result::Result::Ok(context) =
				::aidoku::imports::std::read::<::aidoku::PageContext>(context_descriptor)
			{
				Some(context)
			} else {
				return -2;
			};

			let mut result = __source().get_image_request(url, context);
			if let Ok(request) = result.as_mut() {
				request.should_close = false;
			}
			__handle_result(result.map(|r| r.rid))
		}
	};

	(@single PageDescriptionProvider) => {
		#[no_mangle]
		#[export_name = "get_page_description"]
		pub unsafe extern "C" fn __wasm_get_page_description(page_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(page) =
				::aidoku::imports::std::read::<::aidoku::Page>(page_descriptor)
			else {
				return -1;
			};

			let result = __source().get_page_description(page);
			__handle_result(result)
		}
	};

	(@single AlternateCoverProvider) => {
		#[no_mangle]
		#[export_name = "get_alternate_covers"]
		pub unsafe extern "C" fn __wasm_get_alternate_covers(manga_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(manga) =
				::aidoku::imports::std::read::<::aidoku::Manga>(manga_descriptor)
			else {
				return -1;
			};
			let result = __source().get_alternate_covers(manga);
			__handle_result(result)
		}
	};

	(@single BaseUrlProvider) => {
		#[no_mangle]
		#[export_name = "get_base_url"]
		pub unsafe extern "C" fn __wasm_get_base_url() -> i32 {
			let result = __source().get_base_url();
			__handle_result(result)
		}
	};

	(@single NotificationHandler) => {
		#[no_mangle]
		#[export_name = "handle_notification"]
		pub unsafe extern "C" fn __wasm_handle_notification(string_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(notification) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(string_descriptor)
			else {
				return -1;
			};
			__source().handle_notification(notification);
			return 0;
		}
	};

	(@single DeepLinkHandler) => {
		#[no_mangle]
		#[export_name = "handle_deep_link"]
		pub unsafe extern "C" fn __wasm_handle_deep_link(string_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(url) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(string_descriptor)
			else {
				return -1;
			};
			let result = __source().handle_deep_link(url);
			__handle_result(result)
		}
	};

	(@single BasicLoginHandler) => {
		#[no_mangle]
		#[export_name = "handle_basic_login"]
		pub unsafe extern "C" fn __wasm_handle_basic_login(
			key_descriptor: i32,
			username_descriptor: i32,
			password_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(key) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(key_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(username) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(username_descriptor)
			else {
				return -2;
			};
			let ::core::result::Result::Ok(password) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(password_descriptor)
			else {
				return -3;
			};
			let result = __source().handle_basic_login(key, username, password);
			__handle_result(result)
		}
	};

	(@single WebLoginHandler) => {
		#[no_mangle]
		#[export_name = "handle_web_login"]
		pub unsafe extern "C" fn __wasm_handle_web_login(
			key_descriptor: i32,
			keys_descriptor: i32,
			values_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(key) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(key_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(keys) = ::aidoku::imports::std::read::<
				::aidoku::alloc::Vec<::aidoku::alloc::String>,
			>(keys_descriptor) else {
				return -2;
			};
			let ::core::result::Result::Ok(values) = ::aidoku::imports::std::read::<
				::aidoku::alloc::Vec<::aidoku::alloc::String>,
			>(values_descriptor) else {
				return -3;
			};
			let result = __source().handle_web_login(key, keys.into_iter().zip(values).collect());
			__handle_result(result)
		}
	};

	(@single MigrationHandler) => {
		#[no_mangle]
		#[export_name = "handle_id_migration"]
		pub unsafe extern "C" fn __wasm_handle_id_migration(id_descriptor: i32, id_kind: i32) -> i32 {
			let ::core::result::Result::Ok(id) =
				::aidoku::imports::std::read::<::aidoku::alloc::String>(id_descriptor)
			else {
				return -1;
			};
			let ::core::option::Option::Some(kind) = ::aidoku::IdKind::from(id_kind) else {
				return -2;
			};
			let result = __source().handle_id_migration(id, kind);
			__handle_result(result)
		}
	};
}
