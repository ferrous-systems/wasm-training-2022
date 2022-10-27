# Handling requests

A Compute@Edge application follows a simple request-response model:
The main function of the application receives a `Request` object as an argument,
and produces a `Response` object or an `Error`.

✅ Write a basic handler that returns "Hello World" when `/` is accessed.

```rust
use fastly::http::{Method, StatusCode};
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match (req.get_method(), req.get_path()) {
        (&Method::GET, "/") => {
            Ok(Response::from_status(StatusCode::OK).with_body_text_plain("Hello World!\n"))
        }

        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
```

This uses the `fastly` crate.
Documentation is available at [docs.rs/fastly](https://docs.rs/fastly/0.8.9/fastly/).

✅ Run the project locally:

```
fastly compute serve
```

Your application should be reachable at <http://127.0.0.1:7676/>.
