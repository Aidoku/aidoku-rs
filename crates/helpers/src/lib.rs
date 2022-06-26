#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};

fn internal_encode_uri<T: AsRef<str> + From<String>>(url: T, charset: &[u8]) -> T {
    let url = url.as_ref();
    let bytes = url.as_bytes();
    let hex = "0123456789ABCDEF".as_bytes();

    let mut result: Vec<u8> = Vec::with_capacity(url.len() * 3);

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
    String::from_utf8(result).unwrap_or_default().into()
}

/// Percent-encode an entire URI string. Valid UTF-8 string only!
/// Any character that is not ASCII alphanumeric or one of
/// `~!@#$&*()=:/,;?+'` will be encoded.
///
/// # Examples
/// ```
/// assert_eq!(
///     encode_uri("http://www.example.org/a file with spaces.html"),
///     "http://www.example.org/a%20file%20with%20spaces.html",
/// )
/// ```
pub fn encode_uri<T: AsRef<str> + From<String>>(url: T) -> T {
    internal_encode_uri(url, "~!@#$&*()=:/,;?+'".as_bytes())
}

/// Percent-encode a URI component. Valid UTF-8 string only!
/// Any character that is not ASCII alphanumeric or one of `~!*()'`
/// will be encoded.
///
/// # Examples
/// ```
/// assert_eq!(
///     encode_uri_component(";,/?:@&=+$"),
///     "%3B%2C%2F%3F%3A%40%26%3D%2B%24",
/// )
/// ```
pub fn encode_uri_component<T: AsRef<str> + From<String>>(url: T) -> T {
    internal_encode_uri(url, "~!*()'".as_bytes())
}
