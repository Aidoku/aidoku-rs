//! Create and send HTTP requests.
type Rid = i32;

use crate::{
    alloc::{string::String, vec::Vec},
    error::{AidokuError, AidokuErrorKind, NodeError, Result},
    html::Node,
    std::{Rid as ValueRid, StringRef, ValueRef}, Kind,
};

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
    #[link_name = "init"]
    fn request_init(method: HttpMethod) -> Rid;
    #[link_name = "send"]
    fn request_send(rd: Rid);
    #[link_name = "close"]
    fn request_close(rd: Rid);

    #[link_name = "set_url"]
    fn request_set_url(rd: Rid, value: *const u8, len: usize);
    #[link_name = "set_header"]
    fn request_set_header(rd: Rid, key: *const u8, key_len: usize, val: *const u8, val_len: usize);
    #[link_name = "set_body"]
    fn request_set_body(rd: Rid, value: *const u8, len: usize);

    #[link_name = "get_url"]
    fn request_get_url(rd: Rid) -> Rid;
    #[link_name = "get_data"]
    fn request_get_data(rd: Rid, buffer: *mut u8, size: usize);
    #[link_name = "get_data_size"]
    fn request_get_data_size(rd: Rid) -> usize;
    #[link_name = "get_header"]
    fn request_get_header(rd: Rid, key: *const u8, key_len: usize) -> Rid;
    #[link_name = "get_status_code"]
    fn request_get_status_code(rd: Rid) -> i32;

    #[link_name = "json"]
    fn request_json(rd: Rid) -> ValueRid;
    #[link_name = "html"]
    fn request_html(rd: Rid) -> ValueRid;

    #[link_name = "set_rate_limit"]
    fn request_set_rate_limit(rate_limit: i32);
    #[link_name = "set_rate_limit_period"]
    fn request_set_rate_limit_period(seconds: i32);
}

/// Sets the number of requests allowed within a time period.
pub fn set_rate_limit(rate_limit: i32) {
    unsafe { request_set_rate_limit(rate_limit) }
}

/// Sets the rate limiting duration.
pub fn set_rate_limit_period(seconds: i32) {
    unsafe { request_set_rate_limit_period(seconds) }
}

/// Macro for generating convenience HTTP methods, e.g.
/// Request::get, Request::post.
#[doc(hidden)]
macro_rules! convenience_http_methods {
    ($name:ident, $t:expr, $doc:tt) => {
        #[inline]
        #[doc = $doc]
        pub fn $name<T: AsRef<str>>(url: T) -> Self {
            Self::new(url, $t)
        }
    };
}

/// A type that makes a HTTP request.
#[derive(Debug)]
pub struct Request(pub Rid);
impl Request {
    /// Start a new request with a URL and HTTP method
    ///
    /// Usage:
    /// ```rs
    /// Request::new("https://example.com", HttpMethod::Get)
    /// ```
    pub fn new<T: AsRef<str>>(url: T, method: HttpMethod) -> Self {
        let url = url.as_ref();
        unsafe {
            let rd = request_init(method);
            request_set_url(rd, url.as_ptr(), url.len());
            Self(rd)
        }
    }

    convenience_http_methods! { get, HttpMethod::Get, "Start a new GET request with the given URL." }
    convenience_http_methods! { post, HttpMethod::Post, "Start a new POST request with the given URL." }
    convenience_http_methods! { put, HttpMethod::Put, "Start a new PUT request with the given URL." }
    convenience_http_methods! { head, HttpMethod::Head, "Start a new HEAD request with the given URL." }
    convenience_http_methods! { delete, HttpMethod::Delete, "Start a new DELETE request with the given URL." }

    /// Set a header.
    pub fn header<T: AsRef<str>>(self, key: T, val: T) -> Self {
        let key = key.as_ref();
        let val = val.as_ref();
        unsafe {
            request_set_header(self.0, key.as_ptr(), key.len(), val.as_ptr(), val.len());
        };
        self
    }

    /// Set the body's data.
    pub fn body<T: AsRef<[u8]>>(self, data: T) -> Self {
        let data = data.as_ref();
        unsafe { request_set_body(self.0, data.as_ptr(), data.len()) };
        self
    }

    /// Set the URL for the request
    pub fn set_url<T: AsRef<str>>(self, url: T) -> Self {
        let url = url.as_ref();
        unsafe { request_set_url(self.0, url.as_ptr(), url.len()) }
        self
    }

    #[inline]
    pub fn send(&self) {
        unsafe { request_send(self.0) }
    }

    #[inline]
    pub fn close(&self) {
        unsafe { request_close(self.0) }
    }

    /// Get the response's status code
    #[inline]
    pub fn status_code(&self) -> i32 {
        unsafe { request_get_status_code(self.0) }
    }

    /// Get response headers
    pub fn get_header<T: AsRef<str>>(&self, header: T) -> Option<StringRef> {
        let header = header.as_ref();
        let value = ValueRef::new(unsafe {
            request_get_header(self.0, header.as_ptr(), header.len())
        });
        if value.kind() != Kind::String {
            None
        } else {
            Some(StringRef(value))
        }
    }

    /// Get the URL of the request
    pub fn url(&self) -> StringRef {
        let rid = unsafe { request_get_url(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the raw data from the response
    pub fn data(self) -> Vec<u8> {
        self.send();
        let size = unsafe { request_get_data_size(self.0) };
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        unsafe {
            request_get_data(self.0, buffer.as_mut_ptr(), size);
            buffer.set_len(size);
        }
        self.close();
        buffer
    }

    /// Gets the data as a string.
    #[inline]
    pub fn string(self) -> Result<String> {
        let res = String::from_utf8(self.data());
        match res {
            Ok(res) => Ok(res),
            Err(err) => Err(AidokuError::from(err.utf8_error())),
        }
    }

    /// Get the data as JSON
    pub fn json(self) -> Result<ValueRef> {
        self.send();
        let rid = unsafe { request_json(self.0) };
        self.close();
        match rid {
            -1 => Err(AidokuError {
                reason: AidokuErrorKind::JsonParseError,
            }),
            _ => Ok(ValueRef::new(rid)),
        }
    }

    /// Get the data as a [Node](crate::html::Node).
    pub fn html(self) -> Result<Node> {
        self.send();
        let rid = unsafe { request_html(self.0) };
        self.close();
        match rid {
            -1 => Err(AidokuError::from(NodeError::ParseError)),
            _ => Ok(unsafe { Node::from(rid) }),
        }
    }
}
