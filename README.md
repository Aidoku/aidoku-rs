# Aidoku Rust Source API

This repo contains the following crates:
- [aidoku](crates/lib): A wrapper for Aidoku source libraries.
- [aidoku-cli](crates/cli): A command-line utility for Aidoku source development and testing.
- [aidoku-test](crates/test-macro): A crate that allows for exposing tests to `aidoku-test-runner`.
- [aidoku-test-runner](crates/test-runner): A tool for running tests on Aidoku sources via a custom source runner.

## Aidoku Source Development

To get started with Aidoku source development, you'll need two things: Rust and aidoku-cli.

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/). For aidoku-cli, run the following command after installing Rust:

```sh
cargo install --git https://github.com/Aidoku/aidoku-rs --branch next aidoku-cli
```

Then, create a new source project by running `aidoku init`.

A more detailed guide will be coming soon, once APIs are finalized for the Aidoku 0.7 release.
