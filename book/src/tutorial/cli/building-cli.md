# Building and running with `wasmtime`

✅ You can build for the `wasm32-wasi` manually like this:

```
cargo build --target wasm32-wasi
```

This should create a file `target/wasm32-wasi/debug/rustagram.wasm`.

✅ Alternatively, if you installed `cargo-wasi` (see [Setup](../../setup.md#wasi-tooling)) you can build the application using `cargo wasi`:

```
cargo wasi build
```

`cargo-wasi` handles additional tooling.
For example it calls `wasm-opt` on the resulting file to further reduce the file size.

✅ Now that the application is built you can run it using `wasmtime`:

```
wasmtime target/wasm32-wasi/debug/rustagram.wasm
```

You should see the message printed:

```
$ wasmtime target/wasm32-wasi/debug/rustagram.wasm
Hello World from wasmtime!
```

✅ `cargo-wasi` is able to run the application for you:

```
cargo wasi run
```

Under the hood it just calls out to `wasmtime` as you did above.
However it can't handle additional arguments, so you will need to run `wasmtime` manually later.
