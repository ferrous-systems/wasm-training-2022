.PHONY: build
build:
	cargo build --release --target=wasm32-unknown-unknown
	mkdir -p app
	wasm-bindgen target/wasm32-unknown-unknown/release/image_filter.wasm \
	    --out-dir app \
	    --target no-modules \
	    --no-typescript
