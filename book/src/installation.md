# Setup

This section describes how to set up the toolchain for compiling Rust programs
to WebAssembly and integrate them into JavaScript.

## The Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and
`cargo`.

[Follow these instructions to install the Rust toolchain.][rust-install]

The Rust and WebAssembly experience is riding the Rust release trains to stable!
That means we don't require any experimental feature flags. However, we do
require Rust 1.30 or newer.
