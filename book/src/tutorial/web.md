# Web

In this tutorial you'll get familiar with:

* Building for the `wasm32-unknown-unknown` target
* Interacting with a WASM application from JavaScript
* `wasm-bindgen` tooling to handle more complex types passed over the boundary

Next we build a web application that processes images client-side in the browser.
No server processing involved.

We re-use the same Rust crate to apply the image filter,
but this time load the image directly from a binary blob.
That binary blob is passed in from the JavaScript side.
