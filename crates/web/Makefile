.PHONY: build
build:
	cargo build --release --target=wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/image_filter.wasm \
	    --out-dir app \
	    --target web \
	    --no-typescript
