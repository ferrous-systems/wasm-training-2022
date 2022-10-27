# Hello World on the web

You already saw the ["Hello World of WebAssembly"](../../wasm-hello-world.md) earlier.
You will now run this on the web without additional tools.

✅ Create a new crate

```rust
cargo new --lib hello-world
cd hello-world
```

✅ Set the crate type to `cdylib` in `Cargo.toml`

```toml
[lib]
crate-type = ["cdylib"]
```

✅ Write the `add` function

```rust
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn add(left: c_int, right: c_int) -> c_int {
    left + right
}
```

✅ Compile it to WebAssembly.

```
cargo build --target
```

This will create `target/wasm32-unknown-unknown/debug/hello_world.wasm`.

✅ Next create an HTML file `index.html`

```html
{{#include ../../../../crates/hello-world/index.html:1:8}}
      <!-- to be filled in -->
{{#include ../../../../crates/hello-world/index.html:17:}}
```

✅ Now we need to load, compile and instantiate the WebAssembly module.
All of this is part of the web API.
[`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/fetch) can load data from URLS,
[`WebAssembly.instantiate()`](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/instantiate) compiles and instantiates the WebAssembly module.

```javascript
{{#include ../../../../crates/hello-world/index.html:9:11}}
```

The result of this is an instance that has accessors for the exported functions of the module.

✅ Call the `add` method on the Wasm module instance.

```javascript
{{#include ../../../../crates/hello-world/index.html:12:14}}
```

✅ Serve your HTML file and the WebAssembly over HTTP.

```
python3 -m http.server
```

or

```
http
```

Open <http://localhost:8000> in your web browser and open the Developer Tools.
In the console you should now see the result:

```
1 + 2 = 3
```
