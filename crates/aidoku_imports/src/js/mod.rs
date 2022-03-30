type JsCtx = i32;
type ValueRef = i32;
mod value;
pub use value::{JsValue, JsValueRef};

#[link(wasm_import_module = "js")]
extern "C" {
    #[link_name = "javascript_create_ctx"]
    fn __wasm_javascript_create_ctx() -> JsCtx;
    #[link_name = "javascript_destroy_ctx"]
    fn __wasm_javascript_destroy_ctx(ctx: JsCtx);

    #[link_name = "javascript_eval"]
    fn __wasm_javascript_eval(ctx: JsCtx, script: *const u8, script_len: usize) -> ValueRef;
    #[link_name = "javascript_eval_url"]
    fn __wasm_javascript_eval_url(ctx: JsCtx, url: *const u8, url_len: usize) -> ValueRef;
}

pub struct JsContext(JsCtx);

impl JsContext {
    pub fn new() -> Self {
        let ctx = unsafe { __wasm_javascript_create_ctx() };
        Self(ctx)
    }

    pub fn eval(&self, script: &str) -> JsValue {
        let js_value = unsafe { __wasm_javascript_eval(self.0, script.as_ptr(), script.len()) };
        JsValue::from(js_value)
    }

    pub fn eval_url(&self, url: &str) -> JsValue {
        let js_value = unsafe { __wasm_javascript_eval_url(self.0, url.as_ptr(), url.len()) };
        JsValue::from(js_value)
    }
}

impl Drop for JsContext {
    fn drop(&mut self) {
        unsafe { __wasm_javascript_destroy_ctx(self.0) };
    }
}
