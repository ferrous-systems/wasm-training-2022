# Running it locally

✅ First build your Rust code to WebAssembly and run `wasm-bindgen` to generate the JavaScript shim.
If you are using the `Makefile` you can now run:

```
make
```

Otherwise run the commands directly:

```
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/image_filter.wasm --out-dir app --target no-modules --no-typescript
```

You should find 2 additional files in the `app` directory:
`image_filter.js` and `image_filter_bg.wasm`.

✅ Serve your application locally using `http`:

```
cd app
http
```

Your application should be reachable at <http://127.0.0.1:8000/>.

Play around with it, upload an image and change filters.
