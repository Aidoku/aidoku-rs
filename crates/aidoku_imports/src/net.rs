type Rid = i32;
use super::std::{Rid as ValueRid, ValueRef};

const BUFFER_CHUNK_SIZE: usize = 0x80;

#[repr(C)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
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
    #[link_name = "request_send"]
    fn __wasm_request_send(rd: Rid);
    #[link_name = "request_get_data"]
    fn __wasm_request_get_data(rd: Rid, buffer: *mut u8, size: usize);
    #[link_name = "request_get_data_size"]
    fn __wasm_request_get_data_size(rd: Rid) -> usize;
    #[link_name = "request_close"]
    fn __wasm_request_close(rd: Rid);
    #[link_name = "request_json"]
    fn __wasm_request_json(rd: Rid) -> ValueRid;
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

    fn send(&self) {
        unsafe { __wasm_request_send(self.0) }
    }

    fn close(&self) {
        unsafe { __wasm_request_close(self.0) }
    }

    /// Get the raw data from the response
    pub fn data<'a>(self) -> Vec<u8> {
        self.send();
        let size = unsafe { __wasm_request_get_data_size(self.0) };
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        let mut offset: usize = 0;
        while offset < size {
            let ending_offset = offset + BUFFER_CHUNK_SIZE;
            let chunk = if ending_offset < size {
                &mut buffer[offset..ending_offset]
            } else {
                &mut buffer[offset..]
            };
            unsafe { __wasm_request_get_data(self.0, chunk.as_mut_ptr(), chunk.len()) }
            offset = ending_offset;
        }
        self.close();
        buffer
    }

    /// Get the data as JSON
    pub fn json(self) -> ValueRef {
        self.send();
        let rid = unsafe { __wasm_request_json(self.0) };
        self.close();
        ValueRef::new(rid)
    }
}
