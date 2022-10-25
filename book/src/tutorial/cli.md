# Command-line interface

In this tutorial you'll get familiar with:

* building for the WASM WASI target
* running on the command-line using `wasmtime`
* re-using existing crates in a WASM application
* `wasmtime`'s file system access grants

We start with a command-line tool that takes an image and a filter name as input.
It applies the given filter to the image and produces an `output.png`.
