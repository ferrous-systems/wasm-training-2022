# Hello World on the web

You already saw the ["Hello World of WebAssembly"](../../wasm-hello-world.md) earlier.
You will now run this on the web without additional tools.

✅ Create a new crate

```console
cargo new --lib hello-world
cd hello-world
```

✅ Set the crate type to `cdylib` in `Cargo.toml`

```toml
[lib]
crate-type = ["cdylib"]
```

✅ Write the `add` function.

```rust
{{#include ../../../../crates/hello-world/src/lib.rs}}
```

The `no_mangle` attribute ensures that the function name lands in the binary as is,
otherwise you couldn't later call it by name.
`extern "C"` ensures it uses the C-compatible ABI, and thus what WebAssembly (and JavaScript) expects.

✅ Compile it to WebAssembly.

```
cargo build --target wasm32-unknown-unknown
```

This will create `target/wasm32-unknown-unknown/debug/hello_world.wasm`.

✅ Next create an HTML file `index.html`

```html
{{#include ../../../../crates/hello-world/index.html:1:8}}
      <!-- to be filled in -->
{{#include ../../../../crates/hello-world/index.html:16:}}
```

✅ Now you need to load, compile and instantiate the WebAssembly module.
All of this is part of the web API.
[`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/fetch) can load data from URLs,
[`WebAssembly.instantiate()`](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/instantiate) compiles and instantiates the WebAssembly module.

```javascript
{{#include ../../../../crates/hello-world/index.html:9:11}}
```

The result of this is an instance that has accessors for the exported functions of the module.

✅ Call the `add` method on the Wasm module instance.

```javascript
{{#include ../../../../crates/hello-world/index.html:12:15}}
```

✅ Serve your HTML file and the WebAssembly over HTTP.

```
http
```

Open <http://localhost:8000> in your web browser and open the Developer Tools.
In the console you should now see the result:

```
1 + 2 = 3
```
