type Rid = usize;
use super::json::Rid as JsonRid;

#[repr(C)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Put,
    Delete,
}

#[link(wasm_import_module = "net")]
extern "C" {
    #[link_name = "request_init"]
    fn __wasm_request_init(method: HttpMethod) -> Rid;
    #[link_name = "request_set_url"]
    fn __wasm_request_set_url(rd: Rid, value: *const u8, len: usize);
    #[link_name = "request_set_header"]
    fn __wasm_request_set_header(
        rd: Rid,
        key: *const u8,
        key_len: usize,
        val: *const u8,
        val_len: usize,
    );
    #[link_name = "request_set_body"]
    fn __wasm_request_set_body(rd: Rid, value: *const u8, len: usize);
    #[link_name = "request_data"]
    fn __wasm_request_data(rd: Rid, size: *mut usize) -> *const u8;

    #[link_name = "request_json"]
    fn __wasm_request_json(rd: Rid) -> JsonRid;
}

/// A type that makes a HTTP request.
pub struct Request(Rid);

impl Request {
    /// Start a new request with a URL and HTTP method
    ///
    /// Usage:
    /// ```rs
    /// Request::new("https://example.com", HttpMethod::Get)
    /// ```
    pub fn new(url: &str, method: HttpMethod) -> Self {
        unsafe {
            let rd = __wasm_request_init(method);
            __wasm_request_set_url(rd, url.as_ptr(), url.len());
            Self(rd)
        }
    }

    /// Set a header
    pub fn header(self, key: &str, val: &str) -> Self {
        unsafe {
            __wasm_request_set_header(self.0, key.as_ptr(), key.len(), val.as_ptr(), val.len());
        };
        self
    }

    /// Set the body's data
    pub fn body(self, data: &[u8]) -> Self {
        unsafe { __wasm_request_set_body(self.0, data.as_ptr(), data.len()) };
        self
    }

    /// Get the raw data from the response
    pub fn data<'a>(self) -> &'a [u8] {
        unsafe {
            let mut len = 0;
            let ptr = __wasm_request_data(self.0, &mut len);
            core::slice::from_raw_parts(ptr, len)
        }
    }
}
