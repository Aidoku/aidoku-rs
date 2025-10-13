//! Module for interacting with user preferences.
use super::{
	std::{encode, free_result, read},
	FFIResult, Ptr,
};
use crate::{
	alloc::{String, Vec},
	imports::std::destroy,
};
use crate::{prelude::format, structs::HashMap};
use serde::{de::DeserializeOwned, Serialize};

#[link(wasm_import_module = "defaults")]
extern "C" {
	fn get(key: *const u8, len: usize) -> FFIResult;
	fn set(key: *const u8, len: usize, kind: u8, value: Ptr) -> FFIResult;
}

/// A default value that can be stored in UserDefaults.
pub enum DefaultValue {
	Bool(bool),
	Int(i32),
	Float(f32),
	String(String),
	StringArray(Vec<String>),
	Null,
	HashMap(HashMap<String, String>),
}

impl DefaultValue {
	fn as_byte(&self) -> u8 {
		match self {
			Self::Bool(_) => 1,
			Self::Int(_) => 2,
			Self::Float(_) => 3,
			Self::String(_) => 4,
			Self::StringArray(_) => 5,
			Self::Null => 6,
			Self::HashMap(_) => 0, // not a valid default value
		}
	}
}

/// Returns the UserDefaults value associated with the specified key.
pub fn defaults_get<Value: DeserializeOwned>(key: &str) -> Option<Value> {
	let rid = unsafe { get(key.as_ptr(), key.len()) };
	if rid < 0 {
		return None;
	}
	let result = read::<Value>(rid).ok();
	unsafe { destroy(rid) };
	result
}

/// Returns a HashMap stored in UserDefaults with the specified key.
pub fn defaults_get_map(key: &str) -> Option<HashMap<String, String>> {
	let keys = defaults_get::<Vec<String>>(&format!("{key}.keys"))?;
	let values = defaults_get::<Vec<String>>(&format!("{key}.values"))?;
	Some(keys.into_iter().zip(values).collect())
}

/// Returns the UserDefaults value associated with the specified key, deserialized into a JSON object.
#[cfg(feature = "json")]
pub fn defaults_get_json<Value: DeserializeOwned>(key: &str) -> super::error::Result<Value> {
	let data: String = defaults_get(key).unwrap_or_default();
	let value = serde_json::from_slice(&data.as_bytes())
		.map_err(|e| super::error::AidokuError::JsonParseError(e))?;
	Ok(value)
}

/// Sets the UserDefaults value of the specified key.
pub fn defaults_set(key: &str, value: DefaultValue) {
	let value_ptr: Ptr = unsafe {
		match value {
			DefaultValue::Bool(ref value) => encode(value),
			DefaultValue::Int(ref value) => encode(value),
			DefaultValue::Float(ref value) => encode(value),
			DefaultValue::String(ref value) => encode(value),
			DefaultValue::StringArray(ref value) => encode(value),
			DefaultValue::Null => 0,
			DefaultValue::HashMap(value) => {
				// handle hashmap specially
				// keys and values are stored separately as string arrays
				let keys = value.keys().cloned().collect::<Vec<_>>();
				let values = keys
					.iter()
					.map(|k| value.get(k).unwrap().clone())
					.collect::<Vec<_>>();
				defaults_set(&format!("{key}.keys"), DefaultValue::StringArray(keys));
				defaults_set(&format!("{key}.values"), DefaultValue::StringArray(values));
				return;
			}
		}
	};
	let kind = value.as_byte();
	unsafe {
		set(key.as_ptr(), key.len(), kind, value_ptr);
	};
	if value_ptr != 0 {
		unsafe { free_result(value_ptr) };
	}
}

/// Sets the UserDefaults value of the specified key with serialized data.
pub fn defaults_set_data<T: Serialize>(key: &str, value: T) {
	let value_ptr: i32 = unsafe { encode(&value) };
	unsafe {
		set(key.as_ptr(), key.len(), 0, value_ptr);
	};
	unsafe { free_result(value_ptr) };
}
