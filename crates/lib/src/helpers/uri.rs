//! Module for encoding URIs.
//!
//! This module encodes a UTF-8 URI string by replacing each instance of
//! certain characters with an escape sequence representing the UTF-8
//! encoding of the character.
use core::fmt::Display;

extern crate alloc;
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `internal_encode_uri` escapes all non-alphanumeric characters not
/// in the `charset` parameter.
///
/// This function is made public for use with a custom unencoded charset.
pub fn internal_encode_uri<T: AsRef<[u8]>>(url: T, charset: T) -> String {
	let bytes = url.as_ref();
	let charset = charset.as_ref();
	let hex = b"0123456789ABCDEF";

	let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 3);

	for &byte in bytes {
		if byte.is_ascii_alphanumeric() || charset.contains(&byte) {
			result.push(byte);
		} else {
			result.push(b'%');
			result.push(hex[(byte >> 4) as usize]);
			result.push(hex[(byte & 0x0F) as usize]);
		}
	}
	String::from_utf8(result).unwrap_or_default()
}

/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `encode_uri` escapes all characters except `a-z A-Z 0-9 ; , / ? : @ & = +
/// $ - _ . ! ~ * ' ( ) #`.
///
/// # Examples
/// ```
/// use aidoku::helpers::uri::encode_uri;
/// assert_eq!(
///     encode_uri("http://www.example.org/a file with spaces.html"),
///     "http://www.example.org/a%20file%20with%20spaces.html",
/// )
/// ```
pub fn encode_uri<T: AsRef<[u8]>>(url: T) -> String {
	internal_encode_uri(url.as_ref(), b";,/?:@&=+$-_.!~*'()#")
}

/// Percent-encode a URI component string that is valid UTF-8.
///
/// `encode_uri_component` escapes all characters except `a-z A-Z 0-9 - _ . !
/// ~ * ' ( )`.
///
/// # Examples
/// ```
/// use aidoku::helpers::uri::encode_uri_component;
/// assert_eq!(
///     encode_uri_component(";,/?:@&=+$"),
///     "%3B%2C%2F%3F%3A%40%26%3D%2B%24",
/// )
/// ```
pub fn encode_uri_component<T: AsRef<[u8]>>(url: T) -> String {
	internal_encode_uri(url.as_ref(), b"-_.!~*'()")
}

/// Alternating, decoded query names and values.
#[derive(Clone, Debug, Default)]
pub struct QueryParameters {
	params: Vec<(String, Option<String>)>,
}

impl QueryParameters {
	#[inline]
	pub fn new() -> Self {
		QueryParameters { params: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		QueryParameters {
			params: Vec::with_capacity(capacity),
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.params.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.params.is_empty()
	}

	/// Percent-encode the query paramter with [encode_uri_component] and
	/// add it to the query string along with a value.
	pub fn push(&mut self, name: &str, value: Option<&str>) {
		self.params
			.push((encode_uri_component(name), value.map(encode_uri_component)));
	}

	/// Percent-encode the query paramter with [encode_uri_component] and
	/// add it to the query string.
	pub fn push_key<K: AsRef<str>>(&mut self, name: K) {
		self.params
			.push((encode_uri_component(name.as_ref()), None));
	}

	/// Add a pre-encoded query parameter to the query string.
	pub fn push_encoded(&mut self, name: &str, value: Option<&str>) {
		self.params
			.push((name.to_string(), value.map(|v| v.to_string())));
	}

	/// Percent-encode the query parameter with [encode_uri_component] and
	/// replace any existing values.
	pub fn set(&mut self, name: &str, value: Option<&str>) {
		self.remove_all(name);
		self.push(name, value);
	}

	/// Replace any existing values with the given pair, without encoding.
	pub fn set_encoded(&mut self, name: &str, value: Option<&str>) {
		self.remove_all(name);
		self.push_encoded(name, value);
	}

	/// Remove all query parameters matching given name.
	pub fn remove_all<T: AsRef<str>>(&mut self, name: T) {
		let name = name.as_ref();
		self.remove_all_encoded(encode_uri_component(name));
	}

	/// Remove all query parameters matching given pre-encoded name.
	pub fn remove_all_encoded<T: AsRef<str>>(&mut self, name: T) {
		let name = name.as_ref();
		self.params.retain(|(n, _)| n != name);
	}
}

impl Display for QueryParameters {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut first = true;
		for (n, v) in &self.params {
			if !first {
				write!(f, "&")?;
			} else {
				first = false;
			}
			write!(f, "{}", n)?;
			if let Some(v) = v {
				write!(f, "={}", v)?;
			}
		}
		Ok(())
	}
}
