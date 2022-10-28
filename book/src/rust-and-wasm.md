# Rust & Wasm

The Rust compiler gained proper asm.js support (a wasm predecessor) using Emscripten some time in 2016
and experimental WebAssembly support shortly after the same year with Rust 1.14 (the `wasm32-unknown-emscripten` target).

The `wasm32-unknown-unknown` target became available on Rust Nightly in November 2017.
`wasm32-wasi` was added in 2019 (initially as `wasm32-unknown-wasi`).
These are the two main targets you will work with.

A [WebAssembly Domain Working Group](https://github.com/rustwasm/) was started within the Rust project in 2018.
Their plan was to drive WebAssembly support in the Rust compiler forward,
create tooling and libraries for Rust & WebAssembly and provide learning material.

Early on WebAssembly tooling was written in Rust,
often to simplify Rust & WebAssembly development,
but sometimes acting as general tooling as well.
Tools such as [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
or [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
became early examples of what great WebAssembly tooling can provide for the ecosystem.

Some WebAssembly runtimes were written in Rust, most notably [wasmtime](https://wasmtime.dev/).
The community started developing libraries and frameworks for WebAssembly development,
e.g. [Yew](https://crates.io/crates/yew), a framework for making client-side single-page apps.

From the get-go Rust was a first-class citizen in the WebAssembly world,
both as a language targeting WebAssembly as well as the language tools and libraries for WebAssembly were written in.
