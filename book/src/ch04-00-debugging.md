# Debugging

Now with a general understanding of what goes into creating and running an Aidoku source, we can
discuss methods for debugging issues.

## Logging

The age-old solution for debugging is a bunch of print statements. Like any standard Rust program,
you can use the `println!` macro to do this, but there are a few options for viewing the resulting
logs. The standard method is to use the "Display Logs" button in the Advanced app settings. As
mentioned in the previous chapter, logs will also be displayed in the Xcode log viewer if the
debugger is attached to Aidoku.

However, you may also notice the "Log Server" setting above the "Display Logs" button. Aidoku will
post any log messages to the provided server URL, and you can run your own server with `aidoku`:

```sh
aidoku logcat
```

Enter the URL given to you by the command execution in the "Log Server" input field on a device
connected to the same network and all logs should be streamed.

## Writing Tests

While Rust programs targeting WebAssembly programs don't support the standard Rust test runner,
aidoku-rs provides a custom test runner that simulates Aidoku's environment. This doesn't provide a
complete set of the available source APIs, and there may be a few differences from the Aidoku app,
but it should be enough for simple tests. To get started, install the test runner with the following
command:

```sh
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-test-runner
```

And then configure the project to use the test runner in the `.cargo/config.toml` file:

```toml
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = "aidoku-test-runner"
```

Projects created with `aidoku init` should already have the `config.toml` file configured and
`aidoku-test` added as a dev dependency. To use this, you write tests as you normally would in Rust,
but attach `#[aidoku_test]` to test functions rather than `#[test]`:

```rust,noplayground#[cfg(test)]
mod test {
	use super::*;
	use aidoku_test::aidoku_test;
	
	#[aidoku_test]
	fn test_js_execution() {
		use aidoku::imports::js::JsContext;
		let context = JsContext::new();
		let result = context.eval("1 + 2");
		assert_eq!(result, Ok(String::from("3")));
	}
}
```

If you run into any issues using the test runner, please
[create an issue](https://github.com/Aidoku/aidoku-rs/issues) on the GitHub repo so we can improve
it.
