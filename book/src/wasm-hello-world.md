# The Hello World of WebAssembly

We will work with Rust throughout this book.
The first "Hello World" application is thus a small Rust function to add 2 numbers together
and return the result.

```rust
{{#include ../../crates/hello-world/src/lib.rs}}
```

WebAssembly is a binary format.
The above function compiled to a WebAssembly module results in the following binary blob (hexdumped).

```hex
00 61 73 6d 01 00 00 00 01 07 01 60 02 7f 7f 01
7f 03 02 01 00 05 03 01 00 10 07 10 02 06 6d 65
6d 6f 72 79 02 00 03 61 64 64 00 00 0a 09 01 07
00 20 00 20 01 6a 0b
```

Along with the binary format there's also the WebAssembly text format, `wat`.
The above module represented as `wat`:

```wasm
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)
  (export "memory" (memory 0))
  (export "add" (func $add))
)
```

_(The `wasm2wat` tool transforms the binary output to its equivalent text format)_

In later chapters of this book you will learn how to write, compile and run these WebAssembly modules in different environments.
