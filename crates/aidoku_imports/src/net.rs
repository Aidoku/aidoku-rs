type NetworkDescriptor = usize;

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
    fn __wasm_request_init(method: HttpMethod) -> NetworkDescriptor;
    #[link_name = "request_set_url"]
    fn __wasm_request_set_url(rd: NetworkDescriptor, value: *const u8, len: usize);
    #[link_name = "request_set_header"]
    fn __wasm_request_set_header(
        rd: NetworkDescriptor,
        key: *const u8,
        key_len: usize,
        val: *const u8,
        val_len: usize,
    );
    #[link_name = "request_set_body"]
    fn __wasm_request_set_body(rd: NetworkDescriptor, value: *const u8, len: usize);
    #[link_name = "request_data"]
    fn __wasm_request_data(rd: NetworkDescriptor, size: *mut usize) -> *const u8;
}

pub struct Request(NetworkDescriptor);

impl Request {
    pub fn new(url: &str, method: HttpMethod) -> Self {
        unsafe {
            let rd = __wasm_request_init(method);
            __wasm_request_set_url(rd, url.as_ptr(), url.len());
            Self(rd)
        }
    }

    pub fn header(self, key: &str, val: &str) -> Self {
        unsafe {
            __wasm_request_set_header(self.0, key.as_ptr(), key.len(), val.as_ptr(), val.len());
        };
        self
    }

    pub fn body(self, data: &[u8]) -> Self {
        unsafe { __wasm_request_set_body(self.0, data.as_ptr(), data.len()) };
        self
    }

    pub fn data<'a>(self) -> &'a [u8] {
        unsafe {
            let mut len = 0;
            let ptr = __wasm_request_data(self.0, &mut len);
            core::slice::from_raw_parts(ptr, len)
        }
    }
}
