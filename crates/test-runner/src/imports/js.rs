use crate::{libs::StoreItem, FFIResult, Ptr, Rid, WasmEnv};
use boa_engine::{JsString, Source};
use wasmer::FunctionEnvMut;

enum Result {
	// Success,
	#[allow(clippy::enum_variant_names)]
	MissingResult,
	InvalidContext,
	InvalidString,
	// InvalidHandler,
	// InvalidRequest,
}

impl From<Result> for i32 {
	fn from(result: Result) -> Self {
		match result {
			// Result::Success => 0,
			Result::MissingResult => -1,
			Result::InvalidContext => -2,
			Result::InvalidString => -3,
			// Result::InvalidHandler => -4,
			// Result::InvalidRequest => -5,
		}
	}
}

pub fn context_create(mut env: FunctionEnvMut<WasmEnv>) -> Rid {
	let context = boa_engine::Context::default();
	env.data_mut()
		.store
		.store(StoreItem::JsContext(Box::new(context)))
}
pub fn context_eval(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	string_ptr: Ptr,
	len: u32,
) -> FFIResult {
	let Ok(string) = env.data().read_string(&env, string_ptr, len) else {
		return Result::InvalidString.into();
	};
	let Some(context) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_js_context())
	else {
		return Result::InvalidContext.into();
	};
	let src = Source::from_bytes(&string);
	let Ok(result) = context.eval(src) else {
		return Result::MissingResult.into();
	};
	let Some(result_string) = result
		.to_string(context)
		.ok()
		.and_then(|s| s.to_std_string().ok())
	else {
		return Result::MissingResult.into();
	};
	env.data_mut().store.store(StoreItem::String(result_string))
}
pub fn context_get(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	string_ptr: u32,
	len: u32,
) -> FFIResult {
	let Ok(string) = env.data().read_string(&env, string_ptr, len) else {
		return Result::InvalidString.into();
	};
	let Some(context) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_js_context())
	else {
		return Result::InvalidContext.into();
	};
	let key: JsString = string.into();
	let Ok(result) = context.global_object().get(key, context) else {
		return Result::MissingResult.into();
	};
	let Some(result_string) = result
		.to_string(context)
		.ok()
		.and_then(|s| s.to_std_string().ok())
	else {
		return Result::MissingResult.into();
	};
	env.data_mut().store.store(StoreItem::String(result_string))
}

pub fn webview_create(_env: FunctionEnvMut<WasmEnv>) -> Rid {
	-1
}
pub fn webview_load(_env: FunctionEnvMut<WasmEnv>, _webview: Rid, _request: Rid) -> FFIResult {
	-1
}
pub fn webview_load_html(
	_env: FunctionEnvMut<WasmEnv>,
	_webview: Rid,
	_string_ptr: u32,
	_len: u32,
	_url_ptr: u32,
	_url_len: u32,
) -> FFIResult {
	-1
}
pub fn webview_wait_for_load(_env: FunctionEnvMut<WasmEnv>, _webview: Rid) -> FFIResult {
	-1
}
pub fn webview_eval(
	_env: FunctionEnvMut<WasmEnv>,
	_webview: Rid,
	_string_ptr: u32,
	_len: u32,
) -> FFIResult {
	-1
}
