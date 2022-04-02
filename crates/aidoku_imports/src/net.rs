type Rid = i32;
use super::std::{Rid as ValueRid, ValueRef};

pub use super::alloc::vec::Vec;

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
    fn request_init(method: HttpMethod) -> Rid;
    fn request_set_url(rd: Rid, value: *const u8, len: usize);
    fn request_set_header(rd: Rid, key: *const u8, key_len: usize, val: *const u8, val_len: usize);
    fn request_set_body(rd: Rid, value: *const u8, len: usize);
    fn request_send(rd: Rid);
    fn request_get_data(rd: Rid, buffer: *mut u8, size: usize);
    fn request_get_data_size(rd: Rid) -> usize;
    fn request_close(rd: Rid);
    fn request_json(rd: Rid) -> ValueRid;
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
            let rd = request_init(method);
            request_set_url(rd, url.as_ptr(), url.len());
            Self(rd)
        }
    }

    /// Set a header
    pub fn header(self, key: &str, val: &str) -> Self {
        unsafe {
            request_set_header(self.0, key.as_ptr(), key.len(), val.as_ptr(), val.len());
        };
        self
    }

    /// Set the body's data
    pub fn body(self, data: &[u8]) -> Self {
        unsafe { request_set_body(self.0, data.as_ptr(), data.len()) };
        self
    }

    fn send(&self) {
        unsafe { request_send(self.0) }
    }

    fn close(&self) {
        unsafe { request_close(self.0) }
    }

    /// Get the raw data from the response
    pub fn data<'a>(self) -> Vec<u8> {
        self.send();
        let size = unsafe { request_get_data_size(self.0) };
        let mut buffer: Vec<u8> = Vec::with_capacity(size);
        let mut offset: usize = 0;
        while offset < size {
            let ending_offset = offset + BUFFER_CHUNK_SIZE;
            let chunk = if ending_offset < size {
                &mut buffer[offset..ending_offset]
            } else {
                &mut buffer[offset..]
            };
            unsafe { request_get_data(self.0, chunk.as_mut_ptr(), chunk.len()) }
            offset = ending_offset;
        }
        self.close();
        buffer
    }

    /// Get the data as JSON
    pub fn json(self) -> ValueRef {
        self.send();
        let rid = unsafe { request_json(self.0) };
        self.close();
        ValueRef::new(rid)
    }
}
