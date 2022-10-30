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
The latest Rust should work best.

[rust-install]: https://www.rust-lang.org/tools/install

### WASM targets

Install the WASM targets:

```
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

### Additional tooling

Some of these are optional.
They make some tasks easier to handle, but it can be done without them.

#### wasmtime

A fast and secure runtime for WebAssembly.

Full installation instructions: <https://docs.wasmtime.dev/cli-install.html>

Linux and macOS users can execute the following:

```
curl https://wasmtime.dev/install.sh -sSf | bash
```

Alternatively, on macOS with `brew`:

```
brew install wasmtime
```

This will download a precompiled version of wasmtime and place it in `$HOME/.wasmtime`,
and update your shell configuration to place the right directory in `PATH`.

Windows users should visit the [releases page][wasmtime-releases]
and download the MSI installer (`wasmtime-v2.0.0-x86_64-windows.msi` for example)
and use that to install.

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

#### wasm-bindgen

Tool to generate JavaScript bindings for a wasm file.

```
cargo install wasm-bindgen-cli
```

#### wasm2wat (optional)

Translate from the binary WebAssembly format back to the text format (also known as a .wat).
Part of the WebAssembly Binary Toolkit (WABT).

macOS:

```
brew install wabt
```

Others:

Download the release from the [WABT release page](https://github.com/WebAssembly/wabt/releases).

#### Fastly CLI (optional)

`fastly` is an open-source command line tool for interacting with the Fastly API.
It can be used to create, build and run Compute@Edge projects locally and deploy them on Fastly.

[Installation instructions](https://developer.fastly.com/learning/tools/cli/#installing).

For macOS:

```
brew install fastly/tap/fastly
```

For Windows and Linux:

Download a release from the [fastly GitHub Release page](https://github.com/fastly/cli/releases/latest).

#### Serving local content over HTTP

"Host These Things Please" (`https`) is a basic http server for serving files in a folder over HTTP locally.

Install it using `cargo`:

```
cargo install https
```

You can later simply use `http` to run it.
