use crate::libs::{DefaultKind, DefaultValue, StoreItem};
use crate::{FFIResult, Ptr, WasmEnv};
use wasmer::FunctionEnvMut;

enum Result {
	Success,
	InvalidKey,
	InvalidValue,
	FailedEncoding,
	FailedDecoding,
}

impl From<Result> for i32 {
	fn from(result: Result) -> Self {
		match result {
			Result::Success => 0,
			Result::InvalidKey => -1,
			Result::InvalidValue => -2,
			Result::FailedEncoding => -3,
			Result::FailedDecoding => -4,
		}
	}
}

pub fn get(mut env: FunctionEnvMut<WasmEnv>, key_ptr: Ptr, len: u32) -> FFIResult {
	let Ok(key) = env.data().read_string(&env, key_ptr, len) else {
		return Result::InvalidKey.into();
	};
	let Some(object) = env.data().defaults.get(&key).cloned() else {
		return Result::InvalidValue.into();
	};
	match object {
		DefaultValue::Data(data) => env.data_mut().store.store(StoreItem::Encoded(data)),
		DefaultValue::Bool(bool) => env
			.data_mut()
			.store
			.store_encoded(&bool)
			.unwrap_or(Result::FailedEncoding.into()),
		DefaultValue::Int(int) => env
			.data_mut()
			.store
			.store_encoded(&int)
			.unwrap_or(Result::FailedEncoding.into()),
		DefaultValue::Float(float) => env
			.data_mut()
			.store
			.store_encoded(&float)
			.unwrap_or(Result::FailedEncoding.into()),
		DefaultValue::String(string) => env
			.data_mut()
			.store
			.store_encoded(&string)
			.unwrap_or(Result::FailedEncoding.into()),
		DefaultValue::StringArray(array) => env
			.data_mut()
			.store
			.store_encoded(&array)
			.unwrap_or(Result::FailedEncoding.into()),
		DefaultValue::Null => Result::InvalidValue.into(),
		DefaultValue::HashMap(_) => Result::InvalidValue.into(),
	}
}

pub fn set(
	mut env: FunctionEnvMut<WasmEnv>,
	key_ptr: Ptr,
	len: u32,
	kind: u8,
	value_ptr: Ptr,
) -> FFIResult {
	let Ok(key) = env.data().read_string(&env, key_ptr, len) else {
		return Result::InvalidKey.into();
	};
	if kind > 6 {
		return Result::InvalidValue.into();
	}

	let default_kind: DefaultKind = kind.into();
	let Ok(value_len) = env.data().read_u32(&env, value_ptr) else {
		return Result::FailedDecoding.into();
	};
	let Ok(data) = env.data().read_bytes(&env, value_ptr, value_len) else {
		return Result::FailedDecoding.into();
	};

	let default_value = match default_kind {
		DefaultKind::Data => Ok(DefaultValue::Data(data)),
		DefaultKind::Bool => postcard::from_bytes(&data).map(DefaultValue::Bool),
		DefaultKind::Int => postcard::from_bytes(&data).map(DefaultValue::Int),
		DefaultKind::Float => postcard::from_bytes(&data).map(DefaultValue::Float),
		DefaultKind::String => postcard::from_bytes(&data).map(DefaultValue::String),
		DefaultKind::StringArray => postcard::from_bytes(&data).map(DefaultValue::StringArray),
		DefaultKind::Null => Ok(DefaultValue::Null),
	};
	let Ok(default_value) = default_value else {
		return Result::FailedDecoding.into();
	};

	env.data_mut().defaults.set(key, default_value);

	Result::Success.into()
}
