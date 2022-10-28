# Basic setup

✅ Create a new Rust project.

```
cargo new image-filter
cd image-filter
```

✅ Set the crate type to `cdylib` in `Cargo.toml`

```toml
[lib]
crate-type = ["cdylib"]
```

✅ To simplify the build later on you can use `make` to build the Rust crate and call `wasm-bindgen` to generate the JavaScript shim. Create a `Makefile` and add this:

```makefile
{{#include ../../../../crates/web/Makefile}}
```

You can also use a shell script to do the same or simply run these commands manually.

✅ To help with debugging and logging add these 3 dependencies to `Cargo.toml`

```toml
[dependencies]
console_error_panic_hook = "0.1.7"
console_log = "0.2.0"
log = "0.4.17"
```

`console_error_panic_hook` ensures that you get Rust's panic message & stack trace in your browser's console.
`console_log` ensures you can use Rust's `log` crate for logging as you are used to.

✅ It's time to set up the above mentioned crates in the module's start function.
Annotate your `main` function with `wasm_bindgen(start)`.

```rust
{{#include ../../../../crates/web/src/lib.rs:2:3}}

{{#include ../../../../crates/web/src/lib.rs:9:14}}
```

_Note: The name of this function actually doesn't matter.
The annotation is what tells `wasm-bindgen` that this becomes the setup function._

✅ You should now be able to compile the Rust code to WebAssembly and use `wasm-bindgen` to generate the JavaScript shim.
If you are using the `Makefile` as above you can now run

```
make
```

Otherwise run the commands directly:

```
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/image_filter.wasm --out-dir app --target web --no-typescript
```

You should find 2 new files in the `app` directory:
`image_filter.js` and `image_filter_bg.wasm`.

---

In the [next chapter](image-filters.md) you will write the few Rust pieces necessary for the image filter application.
After that you build the web frontend to load and run the WebAssembly module.
