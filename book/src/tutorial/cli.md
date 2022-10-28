# Command-line interface

In this tutorial you'll get familiar with:

* Building for the `wasm32-wasi` target
* Running applications on the command-line using `wasmtime`
* Re-using existing crates in a WASM application
* `wasmtime`'s capability-based system

We start with a command-line tool that takes an image and a filter name as input.
It applies the given filter to the image and produces an `output.png`.
