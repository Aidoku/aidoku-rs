# aidoku-test-runner

A tool for running tests on Aidoku sources via a custom source runner.

This features a (nearly) complete Aidoku source runner backed by [wasmer](https://wasmer.io/), barring the following features:

- `send_partial_result`: the functionality doesn't make sense in tests, so we don't need it.
- net module rate limiting: I was lazy.
- html mutating functions: unsure of how to implement this with the `scraper` library.
- js module webview components: not sure how to implement this.
- canvas module's `load_font` function: not sure if I need to save the font file somewhere in order to load it.
- canvas module's image drawing/copying: I was lazy.
- locale handling in `parse_date`: chrono doesn't support this, and I'm not sure if there's a good alternative.

However, I haven't tested most of the functionality yet to be honest. Feel free to make an issue if you encounter any problems.

## Usage

This crate depends on [`aidoku-test`](../test-macro). Follow the instructions there to configure tests in your rust code.

You can also run the binary directly from the command line:

```sh
aidoku-test-runner <path_to_wasm_file>
```
