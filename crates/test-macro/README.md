# aidoku-test

A crate that allows for exposing tests to [aidoku-test-runner](../test-runner).

This crate is based on [webassembly-test](https://github.com/matklad/webassembly-test). It functions the same, except it adds a panic hook to the start of every test. This is necessary in order to log panic information to the source stdout before a panic occurs, since the wasmer execution just gives us a runtime error without any useful information.

## Usage

First, add the following dev dependencies:

```toml
[dev-dependencies]
aidoku = { version = "1", features = ["test"] } # the "test" feature disables the panic handler, allowing tests to be run
aidoku-test = "1"
```

In your rust code, simply attach the `aidoku_test` attribute to any testing function:

```rs
#[cfg(test)]
mod test {
	use aidoku_test::aidoku_test;

	#[aidoku_test]
	fn test_function() {
		assert_eq!(1, 1);
	}
}
```

Additionally, the `aidoku-test-runner` harness is required to run the tests. You can install it by running:

```sh
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-test-runner
```

In `.cargo/config.toml`, add the following:

```toml
[target.wasm32-unknown-unknown]
runner = "aidoku-test-runner"
```

Then, `cargo test` will run the tests whenever you're compiling for wasm.
