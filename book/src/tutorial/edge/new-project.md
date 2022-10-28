# New compute project

## Initialize a new package locally using `fastly`

No Fastly account required for local development.
For the manual way see below.

✅ Create a new compute project from a starter kit.

```console
$ mkdir edge-image-filter
$ cd edge-image-filter
$ fastly compute init
```

Give it a name of your choice.  
When asked for the language to use select "Rust".  
When asked for the Starter kit, use "[5] Empty starter for Rust".

✅ Finally run the project locally

```
fastly compute serve
```

Your application should be reachable at <http://127.0.0.1:7676/>.

---

## Initialize a new package locally.

The `fastly` CLI handles creation of a new package.
It essentially does the below steps.


✅ Create a new project using `cargo`

```console
cargo new edge-image-filter
cd edge-image-filter
```

✅ Add the `fastly` dependency

```console
cargo add fastly
```

Alternatively add it to your `Cargo.toml` under `[dependencies]`:

```toml
fastly = "0.8.6"
```

✅ Add the scaffolding to `src/main.rs`:

```rust
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK))
}
```

✅ You also need a `fastly.toml` file with some configuration. Create that file and add this content:


```toml
authors = ["your@email.com"]
description = ""
language = "rust"
manifest_version = 2
name = "edge-image-filter"
service_id = ""
```

✅ Finally run the project locally

```
fastly compute serve
```

Your application should be reachable at <http://127.0.0.1:7676/>.
