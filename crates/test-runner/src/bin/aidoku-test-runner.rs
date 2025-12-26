use aidoku_test_runner::{imports, libs};
use anyhow::{Result, bail};
use libtest_mimic::{Arguments, Failed, Trial};
use std::process::ExitCode;
use wasmer::*;

use libs::WasmEnv;

fn main() -> Result<ExitCode> {
	let file = match std::env::args().nth(1) {
		Some(it) => it,
		None => {
			bail!("usage: aidoku-test-runner <wasm file>");
		}
	};

	let args = Arguments::from_iter(std::env::args().skip(1)); // skip the test runner executable and use wasm file as executable

	let mut store = Store::default();
	let module = Module::from_file(&store, &file)?;
	let env = FunctionEnv::new(&mut store, WasmEnv::new());
	let imports = imports::generate_imports(&mut store, &env);
	let instance = Instance::new(&mut store, &module, &imports)?;
	{
		let env_mut = env.as_mut(&mut store);
		env_mut.memory = Some(instance.exports.get_memory("memory")?.clone());
	}

	let mut tests = Vec::new();
	for export in module.exports() {
		if let Some(name) = export
			.name()
			.strip_prefix("$aidoku-test$")
			.map(|s| s.to_string())
		{
			let mut ignore = true;
			let name = name.strip_prefix("ignore$").unwrap_or_else(|| {
				ignore = false;
				&name
			});

			let file = file.clone();
			let trial = Trial::test(name, move || run_test(&file, export.name(), args.nocapture))
				.with_ignored_flag(ignore);
			tests.push(trial);
		}
	}

	libtest_mimic::run(&args, tests).exit();
}

fn run_test(file: &str, name: &str, nocapture: bool) -> Result<(), Failed> {
	let mut store = Store::default();
	let module = Module::from_file(&store, file)?;
	let env = FunctionEnv::new(&mut store, WasmEnv::new());
	let imports = imports::generate_imports(&mut store, &env);
	let instance = Instance::new(&mut store, &module, &imports)?;
	{
		let env_mut = env.as_mut(&mut store);
		env_mut.memory = Some(instance.exports.get_memory("memory")?.clone());
	}

	let f = instance
		.exports
		.get_typed_function::<(), ()>(&store, name)?;
	let result = f.call(&mut store);
	match result {
		Ok(_) => {
			// print stdout if not capturing output
			if nocapture {
				print!("{}", env.as_ref(&store).stdout.clone());
			}
			Ok(())
		}
		Err(_) => {
			let stdout = env.as_ref(&store).stdout.clone();
			// print stdout if not capturing output
			if nocapture {
				print!("{}", stdout);
			}
			// remove trailing newline
			Err(stdout
				.strip_suffix("\n")
				.map(|s| s.to_string())
				.unwrap_or(stdout)
				.into())
		}
	}
}
