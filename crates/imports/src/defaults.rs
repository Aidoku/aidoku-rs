use super::std::{Rid, ValueRef};

#[link(wasm_import_module = "defaults")]
extern "C" {
    #[link_name = "get"]
    fn _defaults_get(key: *const u8, len: usize) -> Rid;
    #[link_name = "set"]
    fn _defaults_set(key: *const u8, len: usize, value: Rid);
}

pub fn defaults_get<T: AsRef<str>>(key: T) -> ValueRef {
    let key = key.as_ref();
    let rid = unsafe { _defaults_get(key.as_ptr(), key.len()) };
    ValueRef::new(rid)
}

pub fn defaults_set<T: AsRef<str>>(key: T, value: ValueRef) {
    let key = key.as_ref();
    unsafe { _defaults_set(key.as_ptr(), key.len(), value.0) };
}
