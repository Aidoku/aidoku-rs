//! Module for running JavaScript and managing web views.
use super::{
	net::Request,
	std::{destroy, read_string_and_destroy},
	FFIResult, Rid,
};
use crate::alloc::String;

#[link(wasm_import_module = "js")]
extern "C" {
	fn context_create() -> Rid;
	fn context_eval(context: Rid, string_ptr: *const u8, len: usize) -> FFIResult;
	fn context_get(context: Rid, string_ptr: *const u8, len: usize) -> FFIResult;

	fn webview_create() -> Rid;
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
}

/// Error type for JavaScript operations.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JsError {
	MissingResult,
	InvalidContext,
	InvalidString,
	InvalidHandler,
	InvalidRequest,
}

impl JsError {
	fn from(value: FFIResult) -> Option<Self> {
		match value {
			-1 => Some(Self::MissingResult),
			-2 => Some(Self::InvalidContext),
			-3 => Some(Self::InvalidString),
			-4 => Some(Self::InvalidHandler),
			-5 => Some(Self::InvalidRequest),
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
