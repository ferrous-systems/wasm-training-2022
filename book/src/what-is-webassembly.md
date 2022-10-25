# What is WebAssembly?

```rust
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn add(left: c_int, right: c_int) -> c_int {
    left + right
}
```

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
