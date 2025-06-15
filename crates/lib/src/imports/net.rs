//! Module for creating and sending HTTP requests.
use super::{
	canvas::ImageRef,
	error::AidokuError,
	html::Document,
	std::{destroy, read_string_and_destroy},
	FFIResult, Rid,
};
use crate::alloc::{String, Vec};
use serde::{ser::SerializeStruct, Serialize};

/// An HTTP request method.
#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HttpMethod {
	Get,
	Post,
	Put,
	Head,
	Delete,
}

#[link(wasm_import_module = "net")]
extern "C" {
	fn init(method: HttpMethod) -> Rid;
	fn send(rid: Rid) -> FFIResult;
	fn send_all(rd: *mut Rid, len: usize) -> FFIResult;

	fn set_url(rid: Rid, value: *const u8, len: usize) -> FFIResult;
	fn set_header(
		rid: Rid,
		key: *const u8,
		key_len: usize,
		val: *const u8,
		val_len: usize,
	) -> FFIResult;
	fn set_body(rid: Rid, value: *const u8, len: usize) -> FFIResult;

	fn data_len(rid: Rid) -> FFIResult;
	fn read_data(rid: Rid, buffer: *mut u8, size: usize) -> FFIResult;
	fn get_image(rid: Rid) -> FFIResult;
	fn get_header(rid: Rid, key: *const u8, key_len: usize) -> FFIResult;
	fn get_status_code(rid: Rid) -> FFIResult;
	fn html(rid: Rid) -> FFIResult;

	#[link_name = "set_rate_limit"]
	fn net_set_rate_limit(permits: i32, period: i32, unit: i32);
}

/// A time unit for rate limiting.
pub enum TimeUnit {
	Seconds,
	Minutes,
	Hours,
}

impl From<TimeUnit> for i32 {
	fn from(unit: TimeUnit) -> i32 {
		match unit {
			TimeUnit::Seconds => 0,
			TimeUnit::Minutes => 1,
			TimeUnit::Hours => 2,
		}
	}
}

/// Error type for network requests.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RequestError {
	InvalidDescriptor,
	InvalidString,
	InvalidMethod,
	InvalidUrl,
	InvalidHtml,
	InvalidBufferSize,
	MissingData,
	MissingResponse,
	MissingUrl,
	RequestError,
	FailedMemoryWrite,
	NotAnImage,
	Closed,
}

impl RequestError {
	fn from(value: i32) -> Option<Self> {
		match value {
			-1 => Some(Self::InvalidDescriptor),
			-2 => Some(Self::InvalidString),
			-3 => Some(Self::InvalidMethod),
			-4 => Some(Self::InvalidUrl),
			-5 => Some(Self::InvalidHtml),
			-6 => Some(Self::InvalidBufferSize),
			-7 => Some(Self::MissingData),
			-8 => Some(Self::MissingResponse),
			-9 => Some(Self::MissingUrl),
			-10 => Some(Self::RequestError),
			-11 => Some(Self::FailedMemoryWrite),
			-12 => Some(Self::NotAnImage),
			_ => None,
		}
	}
}

/// Macro for generating convenience HTTP methods, e.g.
/// Request::get, Request::post.
#[doc(hidden)]
macro_rules! convenience_http_methods {
	($name:ident, $t:expr, $doc:tt) => {
		#[inline]
		#[doc = $doc]
		pub fn $name<T: AsRef<str>>(url: T) -> Result<Self, RequestError> {
			Self::new(url, $t)
		}
	};
}

/// A type that makes a HTTP request.
#[derive(Debug)]
pub struct Request {
	/// The reference id for the request.
	///
	/// This property is exposed for the functions that the [register_source](crate::register_source)
	/// macro generates and should not be used directly.
	pub rid: Rid,
	http_method: HttpMethod,
	url: Option<String>,
	sent: bool,
	/// Whether the request has been closed and destroyed.
	///
	/// This property is exposed for the functions that the [register_source](crate::register_source)
	/// macro generates and should not be used directly.
	pub forgotten: bool,
	/// The stored request response data.
	pub data: Option<Vec<u8>>,
}

impl Request {
	/// Create a new request with a URL and HTTP method.
	///
	/// Returns an error if the provided URL is invalid.
	///
	/// # Examples
	///
	/// ```ignore
	/// use aidoku::imports::net::{HttpMethod, Request};
	/// Request::new("https://example.com", HttpMethod::Get).unwrap();
	/// ```
	pub fn new<T: AsRef<str>>(url: T, http_method: HttpMethod) -> Result<Self, RequestError> {
		let url = url.as_ref();
		unsafe {
			let rid = init(http_method);
			let mut request = Self {
				rid,
				http_method,
				url: None,
				sent: false,
				forgotten: false,
				data: None,
			};
			request.set_url(url)?;
			Ok(request)
		}
	}

	convenience_http_methods! { get, HttpMethod::Get, "Create a new GET request with the given URL." }
	convenience_http_methods! { post, HttpMethod::Post, "Create a new POST request with the given URL." }
	convenience_http_methods! { put, HttpMethod::Put, "Create a new PUT request with the given URL." }
	convenience_http_methods! { head, HttpMethod::Head, "Create a new HEAD request with the given URL." }
	convenience_http_methods! { delete, HttpMethod::Delete, "Create a new DELETE request with the given URL." }

	/// Send multiple requests in parallel, and wait for all of them to finish.
	pub fn send_all<I>(requests: I) -> Vec<Result<Request, RequestError>>
	where
		I: IntoIterator<Item = Request>,
	{
		let mut ids: Vec<i32> = Vec::new();
		// mark all requests as sent and add their IDs to the vector
		let requests = requests
			.into_iter()
			.map(|mut r| {
				r.sent = true;
				ids.push(r.rid);
				Ok(r)
			})
			.collect();

		let result = unsafe { send_all(ids.as_mut_ptr(), ids.len()) };

		if result == 0 {
			// success, resturn result
			requests
		} else {
			// one or more of the requests failed
			// the error codes are stored in the ids vector
			let mut idx = 0;
			requests
				.into_iter()
				.map(|request| {
					let error = ids.get(idx).and_then(|id| RequestError::from(*id));
					let result = match error {
						Some(e) => Err(e),
						None => request,
					};
					idx += 1;
					result
				})
				.collect()
		}
	}

	/// Set an HTTP header in a builder.
	pub fn header<T: AsRef<str>>(mut self, key: T, val: T) -> Self {
		self.set_header(key, val);
		self
	}

	/// Set an HTTP header.
	pub fn set_header<T: AsRef<str>>(&mut self, key: T, val: T) {
		let key = key.as_ref();
		let val = val.as_ref();
		unsafe {
			set_header(self.rid, key.as_ptr(), key.len(), val.as_ptr(), val.len());
		};
	}

	/// Set the HTTP body data in a builder.
	pub fn body<T: AsRef<[u8]>>(self, data: T) -> Self {
		let data = data.as_ref();
		unsafe { set_body(self.rid, data.as_ptr(), data.len()) };
		self
	}

	/// Set the URL for the request.
	pub fn set_url<T: AsRef<str>>(&mut self, url: T) -> Result<(), RequestError> {
		if self.forgotten {
			return Err(RequestError::Closed);
		}
		let url = url.as_ref();
		self.url = Some(String::from(url));
		let result = unsafe { set_url(self.rid, url.as_ptr(), url.len()) };
		if let Some(error) = RequestError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Get the URL of the request.
	pub fn url(&self) -> Option<&String> {
		self.url.as_ref()
	}

	/// Send the request.
	#[inline]
	pub fn send(&mut self) -> Result<(), RequestError> {
		if self.sent {
			return Ok(());
		}
		if self.forgotten {
			return Err(RequestError::Closed);
		}
		let result = unsafe { send(self.rid) };
		self.sent = true;
		if let Some(error) = RequestError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Get the response's status code.
	#[inline]
	pub fn status_code(&self) -> i32 {
		if !self.sent {
			return -1;
		}
		unsafe { get_status_code(self.rid) }
	}

	/// Get a response HTTP header.
	pub fn get_header<T: AsRef<str>>(&self, header: T) -> Option<String> {
		let header = header.as_ref();
		let rid = unsafe { get_header(self.rid, header.as_ptr(), header.len()) };
		if rid < 0 {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the raw data from the response, closing the request.
	pub fn data(mut self) -> Result<Vec<u8>, RequestError> {
		let result = self.get_data();
		self.close();
		result
	}

	/// Get the raw data from the response.
	pub fn get_data(&mut self) -> Result<Vec<u8>, RequestError> {
		if self.forgotten {
			return Err(RequestError::Closed);
		}
		if !self.sent {
			self.send()?;
		}
		let size = unsafe { data_len(self.rid) };
		if let Some(error) = RequestError::from(size) {
			return Err(error);
		}
		let size = size as usize;
		let mut buffer: Vec<u8> = Vec::with_capacity(size);
		unsafe {
			let result = read_data(self.rid, buffer.as_mut_ptr(), size);
			if let Some(error) = RequestError::from(result) {
				return Err(error);
			}
			buffer.set_len(size);
		}
		Ok(buffer)
	}

	/// Gets the response data as an image.
	pub fn image(mut self) -> Result<ImageRef, RequestError> {
		if !self.sent {
			self.send()?;
		}
		let result = unsafe { get_image(self.rid) };
		if let Some(error) = RequestError::from(result) {
			Err(error)
		} else {
			Ok(ImageRef::from(result, false))
		}
	}

	/// Gets the response data as a string.
	pub fn string(self) -> Result<String, AidokuError> {
		let res = String::from_utf8(self.data()?);
		match res {
			Ok(res) => Ok(res),
			Err(err) => Err(AidokuError::Utf8Error(err.utf8_error())),
		}
	}

	/// Get the response data as an HTML [Document].
	pub fn html(mut self) -> Result<Document, RequestError> {
		self.send()?;
		let rid = unsafe { html(self.rid) };
		self.close();
		if let Some(error) = RequestError::from(rid) {
			return Err(error);
		}
		Ok(unsafe { Document::from(rid) })
	}

	/// Reset the request, clearing any past response. This allows the request to be reused.
	pub fn reset(&mut self) {
		self.close();
		self.rid = unsafe { init(self.http_method) };
		if let Some(url) = self.url.as_ref() {
			_ = unsafe { set_url(self.rid, url.as_ptr(), url.len()) };
		}
		self.sent = false;
		self.forgotten = false;
		self.data = None;
	}

	#[inline]
	fn close(&mut self) {
		if !self.forgotten {
			if self.rid > 0 {
				unsafe { destroy(self.rid) };
			}
			self.forgotten = true;
		}
	}
}

#[cfg(feature = "json")]
impl Request {
	/// Get the response data as a JSON value. This requires the request to stay in scope so the data can be referenced.
	pub fn json<'a, T>(&'a mut self) -> Result<T, AidokuError>
	where
		T: serde::de::Deserialize<'a>,
	{
		let data = self.get_data()?;
		self.data = Some(data);
		let value = serde_json::from_slice(self.data.as_ref().unwrap())
			.map_err(|_| AidokuError::JsonParseError)?;
		Ok(value)
	}

	/// Get the response data as an owned JSON value.
	pub fn json_owned<T>(self) -> Result<T, AidokuError>
	where
		T: serde::de::DeserializeOwned,
	{
		let data = self.data()?;
		let value = serde_json::from_slice(&data).map_err(|_| AidokuError::JsonParseError)?;
		Ok(value)
	}
}

impl Drop for Request {
	fn drop(&mut self) {
		self.close();
	}
}

impl Serialize for Request {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("Request", 2)?;
		state.serialize_field("rid", &self.rid)?;
		state.serialize_field("sent", &self.sent)?;
		state.end()
	}
}

/// Set the number of requests allowed per a given time period.
///
/// For example, `set_rate_limit(10, 60, TimeUnit::Seconds)` will allow 10 requests per minute.
/// If a request is made while the limit is exceeded, the request will be queued and executed
/// once the period is complete.
pub fn set_rate_limit(permits: i32, period: i32, unit: TimeUnit) {
	unsafe { net_set_rate_limit(permits, period, unit.into()) }
}
