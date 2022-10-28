# WASM & JavaScript

WebAssembly is available to every website now through the JavaScript web API.
It is supported in all recent versions of all major browsers[^1].

The WebAssembly web API is available on the [`WebAssembly` JavaScript object](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface).
The available API allows to compile and instantiate WebAssembly modules,
access exported functionality and access the shared memory block
used to share data between the WebAssembly module and the JavaScript environment.
The [web tutorial chapter](../tutorial/web.md) will guide you through some of the usage later.

---

[^1]: See [caniuse.com](https://caniuse.com/?search=webassembly).
