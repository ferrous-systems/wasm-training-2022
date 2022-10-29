# Final application

You should have this file tree layout:

```console
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── app
│   ├── app.js
│   ├── image_filter.js       <-- generated file
│   ├── image_filter_bg.wasm  <-- generated file
│   └── index.html
└── src
    └── lib.rs
```

To recap your final Rust code should look something like this:

```rust
{{#include ../../../../crates/web/src/lib.rs}}
```

The frontend in HTML:

```html
{{#include ../../../../crates/web/app/index.html}}
```

The JavaScript frontend code:

```javascript
{{#include ../../../../crates/web/app/app.js}}
```

A demo deployment is available at:

<https://tmp.fnordig.de/wasm/image-filter/>

---

Some ideas on what to do next:

* The code `unwrap`s a lot. Introduce some error handling. Can you return an error from your wasm module?
