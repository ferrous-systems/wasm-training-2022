# Image filter application

With the basic setup for the Rust code done you can now write a function that applies the image filter to a given image.

✅ Start by importing the necessary modules and structs from the `rustagram2` crate and the standard library in your `src/lib.rs`.

```rust
{{#include ../../../../crates/web/src/lib.rs:5:7}}
```

✅ Next create a new function. It will get a slice of bytes representing the image and a filter name. It then returns a `Vec<u8>`, a vector of bytes representing the modified image in PNG format.

```rust
{{#include ../../../../crates/web/src/lib.rs:16:17}}
    // (to be filled in)
}
```

The code from the next steps will go in this function.

✅ You previously set up logging, use that and log something to ensure you get the data that you expect.

```rust
{{#include ../../../../crates/web/src/lib.rs:17}}
```

✅ The image data needs to be read from the buffer.
The application allows multiple file formats, luckily the `image` format can guess the format and then decode it.
The [documentation for the `Reader` type](https://docs.rs/image/0.24.4/image/io/struct.Reader.html) goes into some detail.

```rust
{{#include ../../../../crates/web/src/lib.rs:19:23}}
```

For now just `unwrap` on errors.
As you have set up the panic handler you should see it in the browser's console if you hit an error.

✅ As you have done in the CLI application parse the filter name into a [`FilterType`](https://docs.rs/rustagram2/2.0.0/rustagram/enum.FilterType.html).

```rust
{{#include ../../../../crates/web/src/lib.rs:25}}
```

Again if you compile everything at this point you will probably hit a type annotation error.
That is expected and you can observe how this changes as you fill in the rest of the code in the next steps.

✅ You now have everything you need to apply the filter to the decoded image.
This is exactly the same as in the previous tutorial.

```rust
{{#include ../../../../crates/web/src/lib.rs:26}}
```

✅ But now instead of saving that changed image to a file you should store it in a buffer and return that buffer.
[`ImageBuffer#write_to`](https://docs.rs/image/0.24.4/image/struct.ImageBuffer.html#method.write_to) does just that.
Don't forget to specify its format as PNG.

```rust
{{#include ../../../../crates/web/src/lib.rs:27:31}}
```

And that is already all the code you need to be able to apply an image filter to a passed in image.

✅ Again build all code and run `wasm-bindgen` to generate the JavaScript shim.
If you are using the `Makefile` as above you can now run

```
make
```

Otherwise run the commands directly:

```
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/image_filter.wasm --out-dir app --target no-modules --no-typescript
```

The JavaScript shim (`image_filter.js`) and the wasm file (`image_filter_bg.wasm`) in your `app` directory should be updated.

---

In the [next chapter](frontend.md) you will work on the other side of this application:
First the HTML frontend and then the necessary JavaScript code to load and run the WebAssembly module.
