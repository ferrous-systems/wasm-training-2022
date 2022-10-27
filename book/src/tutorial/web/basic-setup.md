# Basic setup

✅ Create a new Rust project.

```
cargo new image-filter
cd image-filter
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

`console_error_panic_hook` ensures that you get Rust's panic message & stacktrace in your browser's console.
`console_log` ensures you can use Rust's `log` crate for logging as you are used to.
