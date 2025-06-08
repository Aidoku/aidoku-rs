use crate::{Ptr, WasmEnv};
use wasmer::FunctionEnvMut;

pub fn abort(mut env: FunctionEnvMut<WasmEnv>) {
	env.data_mut().write_stdout("error: abort called.\n");
}

pub fn print(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) {
	let Ok(str) = env.data().read_string(&env, ptr, len) else {
		env.data_mut()
			.write_stdout("error: failed to read string for printing.\n");
		return;
	};
	env.data_mut().write_stdout(&str);
	env.data_mut().write_stdout("\n");
}

pub fn sleep(_env: FunctionEnvMut<WasmEnv>, seconds: i32) {
	std::thread::sleep(std::time::Duration::from_secs(seconds as u64));
}

pub fn send_partial_result(_env: FunctionEnvMut<WasmEnv>, _value: i32) {
	// leaving this function unimplemented for now since the test runner doesn't use partial results
}
