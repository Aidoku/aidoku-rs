use alloc::{string::String, vec::Vec};

/// Percent-encode an entire URI string that is valid UTF-8.
/// 
/// `internal_encode_uri` escapes all non-alphanumeric characters and characters
/// in the `charset` parameter.
/// 
/// This function is made public for use with a custom unencoded charset.
pub fn internal_encode_uri<T: AsRef<str>>(url: T, charset: &[u8]) -> String {
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
    String::from_utf8(result).unwrap_or_default()
}

/// Percent-encode an entire URI string that is valid UTF-8.
/// 
/// `encode_uri` escapes all characters except `a-z A-Z 0-9 ; , / ? : @ & = + $ - _ . ! ~ * ' ( ) #`.
///
/// # Examples
/// ```
/// use aidoku_helpers::uri::encode_uri;
/// assert_eq!(
///     encode_uri("http://www.example.org/a file with spaces.html"),
///     "http://www.example.org/a%20file%20with%20spaces.html",
/// )
/// ```
pub fn encode_uri<T: AsRef<str>>(url: T) -> String {
    internal_encode_uri(url, ";,/?:@&=+$-_.!~*'()#".as_bytes())
}

/// Percent-encode an entire URI string that is valid UTF-8.
/// 
/// `encode_uri_component` escapes all characters except `a-z A-Z 0-9 - _ . ! ~ * ' ( )`.
///
/// # Examples
/// ```
/// use aidoku_helpers::uri::encode_uri_component;
/// assert_eq!(
///     encode_uri_component(";,/?:@&=+$"),
///     "%3B%2C%2F%3F%3A%40%26%3D%2B%24",
/// )
/// ```
pub fn encode_uri_component<T: AsRef<str>>(url: T) -> String {
    internal_encode_uri(url, "-_.!~*'()".as_bytes())
}
