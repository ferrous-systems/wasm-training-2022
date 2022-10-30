# Building and running with `wasmtime`

✅ You can build for the `wasm32-wasi` manually like this:

```
cargo build --target wasm32-wasi
```

This should create a file `target/wasm32-wasi/debug/rustagram.wasm`.

✅ Now that the application is built you can run it using `wasmtime`:

```
wasmtime target/wasm32-wasi/debug/rustagram.wasm
```

You should see the message printed:

```
$ wasmtime target/wasm32-wasi/debug/rustagram.wasm
Hello World from wasmtime!
```

✅ (Optional) You can transform the generated WebAssembly code into its text representation using `wasm2wat`

```
wasm2wat target/wasm32-wasi/debug/rustagram.wasm
```

_Caution: this produces a lot of output._

You should see something like this:

```
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
...
```

Try to identify your "Hello World" code.
