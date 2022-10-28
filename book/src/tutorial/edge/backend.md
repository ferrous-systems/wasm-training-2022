# Backend

You will now implement the actual logic of this API: the image filter.

✅ Start of with some new imports at the top of your `src/main.rs` file.

For easy error handling:

```rust
use fastly::error::{anyhow, bail};
```

You later need to specify the image's mime type:

```rust
use fastly::mime;
```


✅ In your `main` function match for a  `POST` request on the `/image` path and call a handler function.

```rust
match (req.get_method(), req.get_path()) {
    // (cut)

    (&Method::POST, "/image") => convert_image(req),
```

✅ Create this new handler function, taking in the request and returning a response or an error.

```rust
pub fn convert_image(mut req: Request) -> Result<Response, Error> {
    // (to be filled in)
}
```

✅ Next you need to get the required data from the request. Start with the filter name from the query.

```rust
{{#include ../../../../crates/edge/src/main.rs:31:35}}
```

You can use the `anyhow` macro to create an error from just a string.

✅ Now you can check and read the body from the request.

```rust
{{#include ../../../../crates/edge/src/main.rs:41:46}}
```

✅ You can decode the image data using the `image` crate, which is re-exported from `rustagram`.
The documentation is available at [docs.rs/image](https://docs.rs/image/0.24.4/image/).

Import the modules using the following lines on the top of your `src/main.rs` file.

```rust
{{#include ../../../../crates/edge/src/main.rs:6:7}}
```

✅ Now use the `Reader` type to load the image from the buffer.

```rust
{{#include ../../../../crates/edge/src/main.rs:44:48}}
```

✅ Currently Fastly enforces very small resource limits (memory usage, computation time), so you need to limit the work the application does if you want to deploy it.
The easiest is to scale down the image before applying an image filter.

```rust
{{#include ../../../../crates/edge/src/main.rs:50}}
```

Locally you can skip this if you want.
Larger images just take longer to process.

✅ Now that you have the image and a filter you can apply this filter as before.
Instead of writing the result to a file it should be written to a buffer in PNG format.

```rust
{{#include ../../../../crates/edge/src/main.rs:51:53}}
```

✅ The buffer containing the final image can now be returned as the response.
Don't forget to set the correct content type.

```rust
{{#include ../../../../crates/edge/src/main.rs:55:57}}
```

✅ Run the project locally:

```
fastly compute serve
```

Your application should be reachable at <http://127.0.0.1:7676/>.


✅ In another terminal you can use `curl` to send an image and save the converted file.

```
curl http://127.0.0.1:7676/image?filter=valencia -X POST -H "Content-Type: application/octet-stream" -T skyline.jpg -o result.png
```

---

In the [next chapter](frontend.md) you learn how to build a small web frontend and serve that along your image filter application.
