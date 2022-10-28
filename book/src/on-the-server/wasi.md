# WASI

The WebAssembly specification describes a very limited interface that the environment a WebAssembly module runs in need to provide.
It essentially has 3 important parts:

* Imports. Functionality provided by the environment for use within the WebAssembly code.
* Exports. The functions the WebAssembly module exports, making them callable from the outside.
* Linear Memory. The WebAssembly module has access to a block of linear memory, which it potentially can expand on request. This memory can be read by the host environment as well.

Therefore WebAssembly code is limited to self-contained computation, calls to imported functions and reading and writing from memory.
No default imports are provided and how data is laid out in the linear memory is also unspecified.

And this is where WASI comes in:

> WASI is a modular system interface for WebAssembly.
> As described in the initial announcement, it’s focused on security and portability.
>
> _(via [wasi.dev](https://wasi.dev/))_

WASI is a specification of the interfaces a program can use to communicate with the host environment.
It is up to the host environment how these interfaces are implemented and if additional security mechanisms are enforced.

Rust supports the `wasm32-wasi` target and the Rust standard library is implemented for this target,
allowing for most Rust programs and libraries to just work with this target.
The WebAssembly runtime `wasmtime` implements the required WASI interfaces in a capability-based security model

The [initial announcement for WASI](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/) has a lot more details on how it works.
