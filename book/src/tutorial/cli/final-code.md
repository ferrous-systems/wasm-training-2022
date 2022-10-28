# Final application

To recap your final code should look something like this:

``` rust
{{#include ../../../../crates/cli/src/main.rs}}
```

You can build your code like this:

```
cargo build --target wasm32-wasi
```

And run it with `wasmtime`:

```
wasmtime --dir . target/wasm32-wasi/debug/rustagram.wasm skyline.jpg 1977
```

---

Some ideas on what to do next:

* Run the application natively: `cargo run`. Any complications or differences?
* Inspect the built wasm module using `wasm2wat`. Can you spot the parts of the code that you've written? Can you find the names of all available filters?
* Try some other crate you know. Does it work as-is on WebAssembly/with Wasi?
