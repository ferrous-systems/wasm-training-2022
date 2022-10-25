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
