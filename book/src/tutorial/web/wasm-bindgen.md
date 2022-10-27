# Short intrudction to `wasm-bindgen`

WebAssembly is limited to basic integer and float types,
but does not itself support rich types like strings, objects, enums or closures.
However an instantiated WebAssembly module has access to memory,
where it can place more data.
This block of memory is also accessible by the host side, e.g. the JavaScript environment of a website.
Both sides, the WebAssembly code and the host side, need to agree what bytes in that memory block mean in order to work with them.

`wasm-bindgen` is a tool that can generate the necessary code on both sides
to handle more rich types.
It supports a variety of Rust types, including `String`, `Result` and slices
and allows to export Rust types for use in JavaScript (see [`wasm-bindgen`'s Supported Rust types](https://rustwasm.github.io/docs/wasm-bindgen/reference/types.html)).

---

## How to use

The `wasm-bindgen` CLI utility works on the compiled `.wasm` file.
It supports several different output targets.
For this tutorial we focus only on JavaScript and the `no-modules` target.

```
wasm-bindgen path/to/module.wasm --out-dir app --target no-modules --no-typescript
```

## `#[wasm_bindgen(start)]`

This annotation should be put on a public function.
That function essentially becomes your `start` function,
which gets automatically called when you instantiate the WebAssembly module.

Use this to set up a panic handler and logging.

```rust
#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
}
```

## `#[wasm_bindgen]` on a function

If used without additional attributes this wraps the annotated function
to be exported.
You can use the supported Rust types and `wasm-bindgen` will ensure the conversion happens on either side.

```rust
#[wasm_bindgen]
pub fn say_hello(name: String) -> String {
    format!("Hello, {}", name)
}
```

## `#[wasm_bindgen]` on a struct

Annotating a Rust `struct` makes that struct available on the JavaScript side as an object.

```rust
#[wasm_bindgen]
struct Country {
    string shortcode;
}
```

Methods of that struct need to be annotated to be available in JavaScript, too.

```rust
impl Country {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Country {
        Country { shortcode: "NO".to_string() }
    }

    #[wasm_bindgen(getter)]
    pub fn shortcode(&self) -> String {
        self.shortcode.clone()
    }
}
```

See [On Rust exports](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/index.html) in the `wasm-bindgen` documentation for more.

---

In the next chapter you will start building the image filter application for the web,
using `wasm-bindgen` to help with the rich types.
