use crate::{FFIResult, Ptr, Rid, WasmEnv};
use wasmer::FunctionEnvMut;

enum Result {
	Success,
	InvalidDescriptor,
	FailedMemoryWrite,
}

impl From<Result> for i32 {
	fn from(result: Result) -> i32 {
		match result {
			Result::Success => 0,
			Result::InvalidDescriptor => -1,
			Result::FailedMemoryWrite => -2,
		}
	}
}

pub fn destroy(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) {
	env.data_mut().store.remove(rid);
}

pub fn buffer_len(env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(data) = env.data().store.get(rid).and_then(|item| item.as_encoded()) else {
		return Result::InvalidDescriptor.into();
	};
	data.len() as FFIResult
}

pub fn read_buffer(env: FunctionEnvMut<WasmEnv>, rid: Rid, ptr: Ptr, size: u32) -> FFIResult {
	let Some(data) = env.data().store.get(rid).and_then(|item| item.as_encoded()) else {
		return Result::InvalidDescriptor.into();
	};
	if size as usize <= data.len() {
		let data = data.iter().take(size as usize).copied().collect::<Vec<_>>();
		if env.data().write_buffer(&env, ptr, &data).is_err() {
			Result::FailedMemoryWrite.into()
		} else {
			Result::Success.into()
		}
	} else {
		Result::FailedMemoryWrite.into()
	}
}

pub fn current_date(_env: FunctionEnvMut<WasmEnv>) -> f64 {
	use chrono::Utc;
	Utc::now().timestamp() as f64
}

pub fn utc_offset(_env: FunctionEnvMut<WasmEnv>) -> i64 {
	use chrono::Local;
	Local::now().offset().utc_minus_local() as i64
}
