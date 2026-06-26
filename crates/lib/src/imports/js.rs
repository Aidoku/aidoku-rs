//! Module for running JavaScript and managing web views.
use super::{
	FFIResult, Rid,
	net::Request,
	std::{destroy, read_string_and_destroy},
};
use crate::alloc::String;

#[link(wasm_import_module = "js")]
unsafe extern "C" {
	fn context_create() -> Rid;
	fn context_eval(context: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn context_eval_async(context: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn context_get(context: Rid, string_ptr: *const u8, len: usize) -> FFIResult;

	fn webview_create() -> Rid;
	fn webview_set_rule_list(webview: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn webview_load(webview: Rid, request: Rid) -> FFIResult;
	fn webview_load_html(
		webview: Rid,
		string_ptr: *const u8,
		len: usize,
		url_ptr: *const u8,
		url_len: usize,
	) -> FFIResult;
	fn webview_wait_for_load(webview: Rid) -> FFIResult;
	fn webview_eval(webview: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn webview_eval_async(webview: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn webview_add_user_script(
		webview: Rid,
		string_ptr: *const u8,
		len: usize,
		at_document_end: bool,
		for_main_frame_only: bool,
	) -> FFIResult;
}

/// Error type for JavaScript operations.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JsError {
	MissingResult,
	InvalidContext,
	InvalidString,
	InvalidHandler,
	InvalidRequest,
	InvalidRuleList,
}

impl JsError {
	fn from(value: FFIResult) -> Option<Self> {
		match value {
			-1 => Some(Self::MissingResult),
			-2 => Some(Self::InvalidContext),
			-3 => Some(Self::InvalidString),
			-4 => Some(Self::InvalidHandler),
			-5 => Some(Self::InvalidRequest),
			-6 => Some(Self::InvalidRuleList),
			_ => None,
		}
	}
}

/// A context for evaluating JavaScript code.
pub struct JsContext {
	rid: Rid,
}

impl JsContext {
	/// Creates a new JavaScript context.
	pub fn new() -> Self {
		let rid = unsafe { context_create() };
		Self { rid }
	}

	/// Evaluates JavaScript code in the context.
	pub fn eval(&self, js: &str) -> Result<String, JsError> {
		let js_bytes = js.as_bytes();
		let result = unsafe { context_eval(self.rid, js_bytes.as_ptr(), js_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(read_string_and_destroy(result).unwrap_or_default())
		}
	}

	/// Evaluates asynchronous JavaScript code in the context.
	pub fn eval_async(&self, js: &str) -> Result<String, JsError> {
		let js_bytes = js.as_bytes();
		let result = unsafe { context_eval_async(self.rid, js_bytes.as_ptr(), js_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(read_string_and_destroy(result).unwrap_or_default())
		}
	}

	/// Retrieves the value of a JavaScript variable in the context.
	pub fn get(&self, variable: &str) -> Result<String, JsError> {
		let var_bytes = variable.as_bytes();
		let result = unsafe { context_get(self.rid, var_bytes.as_ptr(), var_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(read_string_and_destroy(result).unwrap_or_default())
		}
	}
}

impl Default for JsContext {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for JsContext {
	fn drop(&mut self) {
		unsafe { destroy(self.rid) }
	}
}

/// A web view that can be used to load web content.
///
/// This web view won't be displayed to the user. It is intended for use in the background.
pub struct WebView {
	rid: Rid,
}

impl WebView {
	/// Creates a new web view.
	pub fn new() -> Self {
		let rid = unsafe { webview_create() };
		Self { rid }
	}

	/// Sets a content rule list for the web view.
	///
	/// For information on formatting the rule list json, see Apple's documentation on
	/// [Creating a content blocker](https://developer.apple.com/documentation/SafariServices/creating-a-content-blocker).
	pub fn set_rule_list(&self, json: &str) -> Result<(), JsError> {
		let json_bytes = json.as_bytes();
		let result =
			unsafe { webview_set_rule_list(self.rid, json_bytes.as_ptr(), json_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Loads a web page in the web view.
	pub fn load(&self, request: Request) -> Result<(), JsError> {
		let request_descriptor = request.rid;
		let result = unsafe { webview_load(self.rid, request_descriptor) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Loads a web page in the web view, blocking until the page is loaded.
	pub fn load_blocking(&self, request: Request) -> Result<(), JsError> {
		self.load(request)?;
		self.wait_for_load();
		Ok(())
	}

	/// Loads the given HTML content in the web view.
	pub fn load_html(&self, html: &str, base_url: Option<&str>) -> Result<(), JsError> {
		let html_bytes = html.as_bytes();
		let url_bytes = base_url.map(|s| s.as_bytes()).unwrap_or_default();
		let result = unsafe {
			webview_load_html(
				self.rid,
				html_bytes.as_ptr(),
				html_bytes.len(),
				url_bytes.as_ptr(),
				url_bytes.len(),
			)
		};
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Loads HTML content in the web view, blocking until the content is loaded.
	pub fn load_html_blocking(&self, html: &str, base_url: Option<&str>) -> Result<(), JsError> {
		self.load_html(html, base_url)?;
		self.wait_for_load();
		Ok(())
	}

	/// Blocks the current thread until the web view is loaded.
	pub fn wait_for_load(&self) {
		unsafe { webview_wait_for_load(self.rid) };
	}

	/// Evaluates JavaScript code in the web view, blocking until the result is available.
	pub fn eval(&self, js: &str) -> Result<String, JsError> {
		let js_bytes = js.as_bytes();
		let result = unsafe { webview_eval(self.rid, js_bytes.as_ptr(), js_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(read_string_and_destroy(result).unwrap_or_default())
		}
	}

	/// Evaluates asynchronous JavaScript code in the web view, blocking until the result is available.
	pub fn eval_async(&self, js: &str) -> Result<String, JsError> {
		let js_bytes = js.as_bytes();
		let result = unsafe { webview_eval_async(self.rid, js_bytes.as_ptr(), js_bytes.len()) };
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(read_string_and_destroy(result).unwrap_or_default())
		}
	}

	/// Adds a user script to the web view.
	pub fn add_user_script(&self, script: WebViewUserScript) -> Result<(), JsError> {
		let source_bytes = script.source.as_bytes();
		let result = unsafe {
			webview_add_user_script(
				self.rid,
				source_bytes.as_ptr(),
				source_bytes.len(),
				script.at_document_end,
				script.for_main_frame_only,
			)
		};
		if let Some(error) = JsError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}
}

impl Default for WebView {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for WebView {
	fn drop(&mut self) {
		unsafe { destroy(self.rid) }
	}
}

#[derive(Default)]
/// An object that represents a script that can be injected into webpages
pub struct WebViewUserScript {
	/// The script source.
	pub source: String,
	/// Whether script should be injected at the end of a document or the start.
	pub at_document_end: bool,
	/// Whether the script should be injected into all frames or just the main frame.
	pub for_main_frame_only: bool,
}

impl WebViewUserScript {
	/// Creates a new user script.
	pub fn new(source: String) -> Self {
		Self {
			source,
			..Default::default()
		}
	}
}
