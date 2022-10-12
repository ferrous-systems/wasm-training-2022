# Setup

This section describes how to set up the toolchain for compiling Rust programs
to WebAssembly and integrate them into JavaScript.

## The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and
`cargo`.

[Follow these instructions to install the Rust toolchain.][rust-install]

Rust and WebAssembly is available on Rust stable.
That means we don't require any experimental feature flags.
However, we do require Rust 1.30 or newer.
TODO: check version

[rust-install]: https://www.rust-lang.org/tools/install

### WASM targets

Install the WASM targets:

```
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

### Additional tooling

Subcommand for Cargo to easily build code for `wasm32-wasi`:

```
cargo install cargo-wasi
```

Install `wasmtime`.
Full instructions: <https://docs.wasmtime.dev/cli-install.html>

Short:

```
curl https://wasmtime.dev/install.sh -sSf | bash
```

Or use a precompiled release from <https://github.com/bytecodealliance/wasmtime/releases>.

Alternatively, on macOs:

```
brew install wasmtime
```
