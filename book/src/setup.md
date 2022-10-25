# Setup

This section describes how to set up the toolchain
for compiling Rust programs to WebAssembly
and integrate them with the different environments we will look at.

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

#### wasi tooling

Subcommand for Cargo to easily build code for `wasm32-wasi`.

```
cargo install cargo-wasi
```

#### wasmtime

A fast and secure runtime for WebAssembly

Full installation instructions: <https://docs.wasmtime.dev/cli-install.html>

Linux and macOS users can execute the following:

```
curl https://wasmtime.dev/install.sh -sSf | bash
```

This will download a precompiled version of wasmtime and place it in `$HOME/.wasmtime`,
and update your shell configuration to place the right directory in `PATH`.

Windows users should visit the [releases page][wasmtime-releases]
and download the MSI installer (`wasmtime-v2.0.0-x86_64-windows.msi` for example)
and use that to install.

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

Alternatively, on macOS:

```
brew install wasmtime
```

#### wasm-bindgen

Tool to generate JavaScript bindings for a wasm file.

```
cargo install wasm-bindgen
```

#### wasm2wat

Translate from the binary WebAssembly format back to the text format (also known as a .wat).
Part of the WebAssembly Binary Toolkit (WABT).

macOS:

```
brew install wabt
```

Others:

Download the release from the [WABT release page](https://github.com/WebAssembly/wabt/releases).
