use crate::{
	libs::{HttpMethod, ImageData, NetRequest, NetResponse, StoreItem},
	FFIResult, Ptr, Rid, WasmEnv,
};
use image::ImageReader;
use reqwest::header::{HeaderName, HeaderValue, USER_AGENT};
use scraper::Html;
use std::{io::Cursor, str::FromStr};
use wasmer::FunctionEnvMut;

const DEFAULT_USER_AGENT: &str = "Aidoku/1 CFNetwork/3826.500.131 Darwin/24.5.0";

enum Result {
	Success,
	InvalidDescriptor,
	InvalidString,
	InvalidMethod,
	// InvalidUrl,
	// InvalidHtml,
	// InvalidBufferSize,
	MissingData,
	MissingResponse,
	// MissingUrl,
	RequestError,
	FailedMemoryWrite,
	NotAnImage,
}

impl From<Result> for i32 {
	fn from(result: Result) -> Self {
		match result {
			Result::Success => 0,
			Result::InvalidDescriptor => -1,
			Result::InvalidString => -2,
			Result::InvalidMethod => -3,
			// Result::InvalidUrl => -4,
			// Result::InvalidHtml => -5,
			// Result::InvalidBufferSize => -6,
			Result::MissingData => -7,
			Result::MissingResponse => -8,
			// Result::MissingUrl => -9,
			Result::RequestError => -10,
			Result::FailedMemoryWrite => -11,
			Result::NotAnImage => -12,
		}
	}
}

pub fn init(mut env: FunctionEnvMut<WasmEnv>, method: u8) -> FFIResult {
	let method = match method {
		0 => HttpMethod::Get,
		1 => HttpMethod::Post,
		2 => HttpMethod::Put,
		3 => HttpMethod::Head,
		4 => HttpMethod::Delete,
		_ => return Result::InvalidMethod.into(),
	};
	let request = NetRequest::new(method);
	env.data_mut()
		.store
		.store(StoreItem::Request(Box::new(request)))
}
fn common_send(env: &mut FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	// add a default user agent if none is provided
	if !request.headers.contains_key(USER_AGENT) {
		let default_ua = HeaderValue::from_static(DEFAULT_USER_AGENT);
		request.headers.insert(USER_AGENT, default_ua);
	}
	// make a blocking request with reqwest
	let Ok(response) = reqwest::blocking::Client::new()
		.request(
			match request.method {
				HttpMethod::Get => reqwest::Method::GET,
				HttpMethod::Post => reqwest::Method::POST,
				HttpMethod::Put => reqwest::Method::PUT,
				HttpMethod::Delete => reqwest::Method::DELETE,
				HttpMethod::Head => reqwest::Method::HEAD,
			},
			request.url.as_ref().unwrap(),
		)
		.headers(request.headers.clone())
		.send()
	else {
		return Result::RequestError.into();
	};
	let status = response.status();
	let headers = response.headers().clone();
	let Ok(bytes) = response.bytes() else {
		return Result::RequestError.into();
	};
	request.response = Some(NetResponse {
		status,
		headers,
		data: bytes.into(),
	});
	Result::Success.into()
}
pub fn send(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	common_send(&mut env, rid)
}
pub fn send_all(mut env: FunctionEnvMut<WasmEnv>, rid_ptr: Ptr, len: u32) -> FFIResult {
	let Ok(rids) = env.data().read_values::<Rid>(&env, rid_ptr, len) else {
		return Result::InvalidDescriptor.into();
	};
	let mut errors = Vec::new();
	let mut was_error = false;
	for rid in rids {
		let result = common_send(&mut env, rid);
		if result != Into::<i32>::into(Result::Success) {
			was_error = true;
		}
		errors.push(result);
	}
	if env.data().write_values(&env, rid_ptr, errors).is_err() {
		Result::FailedMemoryWrite.into()
	} else if was_error {
		Result::RequestError.into()
	} else {
		Result::Success.into()
	}
}

pub fn set_url(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(string) = env.data().read_string(&env, ptr, len) else {
		return Result::InvalidString.into();
	};
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	request.url = Some(string);
	Result::Success.into()
}
pub fn set_header(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	key_ptr: Ptr,
	key_len: u32,
	val_ptr: Ptr,
	val_len: u32,
) -> i32 {
	let Ok(key) = env.data().read_string(&env, key_ptr, key_len) else {
		return Result::InvalidString.into();
	};
	let Ok(val) = env.data().read_string(&env, val_ptr, val_len) else {
		return Result::InvalidString.into();
	};
	let Ok(name) = HeaderName::from_str(&key) else {
		return Result::InvalidString.into();
	};
	let Ok(value) = HeaderValue::from_str(&val) else {
		return Result::InvalidString.into();
	};
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	request.headers.insert(name, value);
	Result::Success.into()
}
pub fn set_body(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, ptr: Ptr, len: u32) -> i32 {
	let Ok(body) = env.data().read_bytes(&env, ptr, len) else {
		return Result::InvalidString.into();
	};
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	request.body = Some(body);
	Result::Success.into()
}

pub fn data_len(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let len = response.data.len();
	request.response = Some(response);
	len as i32
}
pub fn read_data(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, buffer: Ptr, size: u32) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let data = response.data.clone();
	request.response = Some(response);

	if size as usize <= data.len() {
		let data = data.into_iter().take(size as usize).collect::<Vec<_>>();
		if env.data().write_buffer(&env, buffer, &data).is_err() {
			Result::FailedMemoryWrite.into()
		} else {
			Result::Success.into()
		}
	} else {
		Result::FailedMemoryWrite.into()
	}
}
pub fn get_image(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let data = response.data.clone();
	request.response = Some(response);

	let cursor = Cursor::new(data);
	let Some(rgba_img) = ImageReader::new(cursor)
		.with_guessed_format()
		.ok()
		.and_then(|r| r.decode().ok())
		.map(|img| img.to_rgb8())
	else {
		return Result::NotAnImage.into();
	};
	let width = rgba_img.width() as i32;
	let height = rgba_img.height() as i32;
	let data = rgba_img.into_raw();
	let image = ImageData {
		data,
		width,
		height,
	};
	env.data_mut().store.store(StoreItem::ImageData(image))
}
pub fn get_status_code(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let status = response.status;
	request.response = Some(response);
	status.as_u16() as i32
}
pub fn get_header(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	key_ptr: Ptr,
	key_len: u32,
) -> FFIResult {
	let Ok(key) = env.data().read_string(&env, key_ptr, key_len) else {
		return Result::InvalidString.into();
	};
	let Ok(name) = HeaderName::from_str(&key) else {
		return Result::InvalidString.into();
	};
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let headers = response.headers.clone();
	request.response = Some(response);

	let value = headers
		.get_all(&name)
		.iter()
		.map(|value| value.as_bytes())
		.filter_map(|bytes| String::from_utf8(bytes.to_vec()).ok())
		.collect::<Vec<String>>()
		.join(", ");
	if value.is_empty() {
		return Result::MissingData.into();
	};
	env.data_mut().store.store(StoreItem::String(value))
}
pub fn html(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(request) = env
		.data_mut()
		.store
		.get_mut(rid)
		.and_then(|item| item.as_request())
	else {
		return Result::InvalidDescriptor.into();
	};
	let Some(response) = request.response.take() else {
		return Result::MissingResponse.into();
	};
	let data = response.data.clone();
	request.response = Some(response);
	let text = match String::from_utf8(data) {
		Ok(s) => s,
		Err(_) => return Result::InvalidString.into(),
	};
	let html = Html::parse_document(&text);
	env.data_mut().store.store(StoreItem::Html(html))
}

pub fn set_rate_limit(_env: FunctionEnvMut<WasmEnv>, _permits: i32, _period: i32, _unit: i32) {
	// leaving this function unimplemented for now
}
