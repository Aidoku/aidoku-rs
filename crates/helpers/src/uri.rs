//! Module for encoding URIs.
//!
//! This module encodes a UTF-8 URI string by replacing each instance of
//! certain characters with an escape sequence representing the UTF-8
//! encoding of the character.
use core::fmt::Display;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `internal_encode_uri` escapes all non-alphanumeric characters and
/// characters in the `charset` parameter.
///
/// This function is made public for use with a custom unencoded charset.
pub fn internal_encode_uri<T: AsRef<[u8]>>(url: T, charset: T) -> String {
    let bytes = url.as_ref();
    let charset = charset.as_ref();
    let hex = "0123456789ABCDEF".as_bytes();

    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 3);

    for byte in bytes {
        let curr = *byte;
        if curr.is_ascii_alphanumeric() || charset.contains(&curr) {
            result.push(curr);
        } else {
            result.push(b'%');
            result.push(hex[curr as usize >> 4]);
            result.push(hex[curr as usize & 15]);
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
/// use aidoku_helpers::uri::encode_uri;
/// assert_eq!(
///     encode_uri("http://www.example.org/a file with spaces.html"),
///     "http://www.example.org/a%20file%20with%20spaces.html",
/// )
/// ```
pub fn encode_uri<T: AsRef<[u8]>>(url: T) -> String {
    internal_encode_uri(url.as_ref(), b";,/?:@&=+$-_.!~*'()#")
}

/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `encode_uri_component` escapes all characters except `a-z A-Z 0-9 - _ . !
/// ~ * ' ( )`.
///
/// # Examples
/// ```
/// use aidoku_helpers::uri::encode_uri_component;
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
    params: Vec<Option<String>>,
}

impl QueryParameters {
    #[inline]
    pub fn new() -> Self {
        QueryParameters { params: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        QueryParameters {
            params: Vec::with_capacity(capacity * 2),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.params.len() / 2
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Percent-encode the query paramter with [encode_uri_component] and
    /// add it to the query string.
    pub fn push<T: AsRef<str>>(&mut self, name: T, value: Option<T>) {
        self.params.push(Some(encode_uri_component(name.as_ref())));
        self.params
            .push(value.map(|v| encode_uri_component(v.as_ref())));
    }

    /// Add a pre-encoded query parameter to the query string.
    pub fn push_encoded<T: AsRef<str>>(&mut self, name: T, value: Option<T>) {
        self.params.push(Some(name.as_ref().to_string()));
        self.params.push(value.map(|v| v.as_ref().to_string()));
    }

    /// Percent-encode the query parameter with [encode_uri_component] and
    /// replace any existing values.
    pub fn set<T: AsRef<str>>(&mut self, name: T, value: Option<T>) {
        self.remove_all(&name);
        self.push(name, value);
    }

    /// Replace any existing values with the given pair.
    pub fn set_encoded<T: AsRef<str>>(&mut self, name: T, value: Option<T>) {
        self.remove_all(&name);
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
        for i in (0..self.params.len()).step_by(2) {
            if let Some(ref param_name) = &self.params.get(i) 
                && let Some(ref param_name) = param_name 
                && name == param_name {
                self.params.remove(i);
                self.params.remove(i);
            }
        }
    }
}

impl Display for QueryParameters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first_pair = true;
        for i in (0..self.params.len()).step_by(2) {
            if let Some(ref param_name) = &self.params[i] {
                if first_pair {
                    first_pair = false
                } else {
                    write!(f, "&")?;
                }
                write!(f, "{param_name}")?;
                if let Some(ref param_value) = &self.params[i + 1] {
                    write!(f, "={param_value}")?;
                }
            }
        }
        Ok(())
    }
}
