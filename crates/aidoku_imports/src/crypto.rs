#[repr(C)]
pub enum Algorithm {
    Aes128,
    Des,
    TripleDES,
    Cast,
    Rc4,
    Rc2,
}

type Options = u32;

#[link(wasm_import_module = "crypto")]
extern "C" {
    #[link_name = "encrypt"]
    fn __wasm_encrypt(
        algorithm: Algorithm,
        options: Options,
        key: *const u8,
        key_len: usize,
        iv: *const u8,
        data_in: *const u8,
        data_in_length: usize,
        data_out: *const u8,
        data_out_available: usize,
        data_out_moved: *mut usize,
    );
    #[link_name = "decrypt"]
    fn __wasm_decrypt(
        algorithm: Algorithm,
        options: Options,
        key: *const u8,
        key_len: usize,
        iv: *const u8,
        data_in: *const u8,
        data_in_length: usize,
        data_out: *const u8,
        data_out_available: usize,
        data_out_moved: *mut usize,
    );
}
