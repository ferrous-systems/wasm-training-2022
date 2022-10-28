# Fastly's Compute@Edge

Fastly is a cloud computing provider and content delivery network (CDN).

Earlier this year they released their Compute@Edge platform.
This platform allows to run WebAssembly code at the Fastly edge.
They chose WebAssembly for exactly the reasons we listed in [the previous chapter](usecases.md#serverless): lightweight sandboxing, per-request isolation and performance.

They released a Rust SDK ([`fastly`](https://docs.rs/fastly/0.8.9/fastly/)),
which provides the necessary integration to read an incoming request and generate an appropriate response.
The host runtime uses [WASI](wasi.md) to provide the necessary system interfaces.
This means that you can write normal Rust code, using most of the Rust standard library and a large number of available Rust crates without issues.
They also support JavaScript and Go as languages on this platform.

You can find the Fastly Compute@Edge documentation at <https://docs.fastly.com/products/compute-at-edge>.

You will learn how to write a small application for this platform in the [Edge computing tutorial](../tutorial/edge.md) later in this book.
