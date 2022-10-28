# Tooling check

## Setup check

✅ Fully restart your terminal (not just open a fresh tab).
✅ Let's check that you have installed Rust.

```console
$ rustc --version
rustc 1.64.0 (a55dd71d5 2022-09-19)
```

```console
$ cargo --version
cargo 1.64.0 (387270bc7 2022-09-16)
```

```console
$ rustup target list --installed
(cut)
wasm32-unknown-unknown
wasm32-wasi
(cut)
```

✅ Let's check that you have installed the tools listed in the previous section (Note: not all are required).

```console
$ wasmtime --version
wasmtime-cli 2.0.0
```

```console
$ cargo wasi --version
cargo-wasi 0.1.26
```

```console
$ wasm-bindgen --version
wasm-bindgen 0.2.83
```

```console
$ wasm2wat --version
1.0.30
```

```console
$ fastly version
Fastly CLI version v4.2.0 (a1e8772)
Built with go version go1.18.6 linux/amd64
Viceroy version: viceroy 0.3.1
```
