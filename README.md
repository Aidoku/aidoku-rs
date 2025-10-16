# Buny Rust Source API

This repo contains the following crates:
- [buny](crates/lib): A wrapper for Buny source libraries.
- [buny-cli](crates/cli): A command-line utility for Buny source development and testing.
- [buny-test](crates/test-macro): A crate that allows for exposing tests to `buny-test-runner`.
- [buny-test-runner](crates/test-runner): A tool for running tests on Buny sources via a custom source runner.

## Buny Source Development

To get started with Buny source development, you'll need two things: Rust and buny-cli.

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/). For buny-cli, run the following command after installing Rust:

```sh
cargo install --git https://github.com/Buny/buny-rs buny-cli
```

Then, create a new source project by running `buny init`.

A more detailed guide will be coming soon, once APIs are finalized for the Buny 0.7 release.
