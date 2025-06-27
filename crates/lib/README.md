# aidoku

The `aidoku` crate contains everything necessary to create a source for Aidoku.

### Crate Features

Default features:

- `talc`: Enables the [talc](https://crates.io/crates/talc) allocator when building for wasm.
- `imports`: Enables the API functions provided to Aidoku sources.
- `helpers`: Enables some helpful additions for source development.

Optional features:

- `json`: Enables deserialization of JSON network responses and default values via [serde_json](https://crates.io/crates/serde_json).
- `test`: Disables the panic handler for use in tests.

### Usage

To use this library to create a source for Aidoku, you need to implement the `Source` trait and register it with the `register_source!` macro.

```rs
struct TestSource;

impl Source for TestSource {
	// implement Source trait methods here
}

// register the source to export wasm functions
register_source!(TestSource, Home);
```
