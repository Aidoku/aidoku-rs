## Tooling

The first step is to install Rust, if not already installed. The recommended method is to do so
using [rustup](https://rustup.rs/), which makes updating and managing your installation easy later
on.

With Rust installed, the next step is to install the proper toolchain for compiling for WebAssembly.
You can do so with the following command:

```sh
rustup install wasm32-unknown-unknown
```

In addition to the Rust tooling, the
[aidoku cli](https://github.com/Aidoku/aidoku-rs/tree/main/crates/cli) provides all the other
necessary functionality for source development. To install it, run:

```sh
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli
```
