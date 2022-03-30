pub type Rid = i32;
use super::iterator::Rid as IterRid;

#[link(wasm_import_module = "json")]
extern "C" {
    #[link_name = "json_parse"]
    fn json_parse(bytes: *const u8, size: usize) -> Rid;
    #[link_name = "json_copy"]
    fn json_copy(rid: Rid) -> Rid;
    #[link_name = "json_destroy"]
    fn json_destroy(rid: Rid) -> Rid;

    // create
    #[link_name = "json_create_array"]
    fn json_create_array() -> Rid;
    #[link_name = "json_create_object"]
    fn json_create_object() -> Rid;
    #[link_name = "json_create_null"]
    fn json_create_null() -> Rid;
    #[link_name = "json_create_string"]
    fn json_create_string(buf: *const u8, len: usize) -> Rid;
    #[link_name = "json_create_bool"]
    fn json_create_bool() -> Rid;
    #[link_name = "json_create_float"]
    fn json_create_float() -> Rid;
    #[link_name = "json_create_int"]
    fn json_create_int() -> Rid;

    // load data
    /// is the value undefined || null?
    #[link_name = "json_is_null"]
    fn json_is_null(ctx: Rid) -> bool;
    #[link_name = "json_read_bool"]
    fn json_read_bool(ctx: Rid) -> bool;
    #[link_name = "json_read_int"]
    fn json_read_int(ctx: Rid) -> i32;
    #[link_name = "json_read_float"]
    fn json_read_float(ctx: Rid) -> f32;
    #[link_name = "json_read_string"]
    fn json_read_string(ctx: Rid, buf: *mut u8, len: usize);
    #[link_name = "json_read_string_len"]
    fn json_read_string_len(ctx: Rid) -> Rid;

    // array
    #[link_name = "json_array_len"]
    fn json_array_len(arr: Rid) -> usize;
    #[link_name = "json_array_get"]
    fn json_array_get(arr: Rid, idx: usize) -> Rid;
    #[link_name = "json_array_set"]
    fn json_array_set(arr: Rid, idx: usize, value: Rid);
    #[link_name = "json_array_append"]
    fn json_array_append(arr: Rid, value: Rid);
    #[link_name = "json_array_remove"]
    fn json_array_remove(arr: Rid, idx: usize);

    // object
    #[link_name = "json_object_len"]
    fn json_object_len(arr: Rid) -> usize;
    #[link_name = "json_object_get"]
    fn json_object_get(arr: Rid, key: *const u8, len: usize) -> Rid;
    #[link_name = "json_object_set"]
    fn json_object_set(arr: Rid, key: *const u8, len: usize, value: Rid);
    #[link_name = "json_object_keys"]
    fn json_object_keys(arr: Rid) -> IterRid;
    #[link_name = "json_object_values"]
    fn json_object_values(arr: Rid) -> IterRid;
    #[link_name = "json_object_remove"]
    fn json_object_remove(arr: Rid, idx: usize);
}
