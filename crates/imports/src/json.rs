use super::std::{Rid, ValueRef};

#[link(wasm_import_module = "json")]
extern "C" {
    #[link_name = "parse"]
    fn json_parse(bytes: *const u8, size: usize) -> Rid;
}

/// Parse JSON data
///
/// Usage:
/// ```rs
/// let val = parse(b"{ \"foo\": 1 }");
/// if let Ok(obj) = val.as_object() {
///     if let Ok(val) =  obj.get("foo") {
///         val // should be 1
///     }
/// }
/// ```
pub fn parse(buf: &[u8]) -> ValueRef {
    let rid = unsafe { json_parse(buf.as_ptr(), buf.len()) };
    ValueRef::new(rid)
}
