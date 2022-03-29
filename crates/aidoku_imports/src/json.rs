type Ctx = i32;

#[link(wasm_import_module = "json")]
extern "C" {
    fn __wasm_json_parse(data: *const u8, len: usize) -> Ctx;
    fn __wasm_json_clone(ctx: Ctx) -> Ctx;
    fn __wasm_json_free(ctx: Ctx);

    fn __wasm_json_cast_string(ctx: Ctx, len: *mut usize) -> *const u8;
    fn __wasm_json_cast_bool(ctx: Ctx) -> bool;
    fn __wasm_json_cast_int(ctx: Ctx) -> i32;
    fn __wasm_json_cast_float(ctx: Ctx) -> f32;
    fn __wasm_json_cast_object(ctx: Ctx) -> Ctx;
    fn __wasm_json_cast_array(ctx: Ctx) -> Ctx;

    fn __wasm_json_object_len(ctx: Ctx);
    fn __wasm_json_object_keys(ctx: Ctx);
    fn __wasm_json_object_values(ctx: Ctx);
    fn __wasm_json_object_delete(ctx: Ctx, key: *const u8, key_len: usize);

    fn __wasm_json_object_get_string(
        ctx: Ctx,
        key: *const u8,
        key_len: usize,
        len: *mut usize,
    ) -> *const u8;
    fn __wasm_json_object_get_bool(ctx: Ctx, key: *const u8, key_len: usize) -> bool;
    fn __wasm_json_object_get_int(ctx: Ctx, key: *const u8, key_len: usize) -> i32;
    fn __wasm_json_object_get_float(ctx: Ctx, key: *const u8, key_len: usize) -> f32;
    fn __wasm_json_object_get_object(ctx: Ctx, key: *const u8, key_len: usize) -> Ctx;
    fn __wasm_json_object_get_array(ctx: Ctx, key: *const u8, key_len: usize) -> Ctx;

    fn __wasm_json_object_set_null(ctx: Ctx, key: *const u8, key_len: usize);
    fn __wasm_json_object_set_string(
        ctx: Ctx,
        key: *const u8,
        key_len: usize,
        value: *const u8,
        len: usize,
    );
    fn __wasm_json_object_set_bool(ctx: Ctx, key: *const u8, key_len: usize, value: bool);
    fn __wasm_json_object_set_int(ctx: Ctx, key: *const u8, key_len: usize, value: i32);
    fn __wasm_json_object_set_float(ctx: Ctx, key: *const u8, key_len: usize, value: f32);
    fn __wasm_json_object_set_object(ctx: Ctx, key: *const u8, key_len: usize, value: Ctx);
    fn __wasm_json_object_set_array(ctx: Ctx, key: *const u8, key_len: usize, value: Ctx);

    fn __wasm_json_array_len(ctx: Ctx);
    fn __wasm_json_array_values(ctx: Ctx);
    fn __wasm_json_array_delete(ctx: Ctx, index: usize);

    fn __wasm_json_array_get_string(ctx: Ctx, index: usize, len: *mut usize) -> *const u8;
    fn __wasm_json_array_get_bool(ctx: Ctx, index: usize) -> bool;
    fn __wasm_json_array_get_int(ctx: Ctx, index: usize) -> i32;
    fn __wasm_json_array_get_float(ctx: Ctx, index: usize) -> f32;
    fn __wasm_json_array_get_object(ctx: Ctx, index: usize) -> Ctx;
    fn __wasm_json_array_get_array(ctx: Ctx, index: usize) -> Ctx;

    fn __wasm_json_array_push_null(ctx: Ctx);
    fn __wasm_json_array_push_string(ctx: Ctx, value: *const u8, len: usize);
    fn __wasm_json_array_push_bool(ctx: Ctx, value: bool);
    fn __wasm_json_array_push_int(ctx: Ctx, value: i32);
    fn __wasm_json_array_push_float(ctx: Ctx, value: f32);
    fn __wasm_json_array_push_object(ctx: Ctx, value: Ctx);
    fn __wasm_json_array_push_array(ctx: Ctx, value: Ctx);
}
