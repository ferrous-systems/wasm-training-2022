# HTML Frontend

If your image filter API is working now you can already use that using `curl` from the command line.
To make it easier to use and test you will now build a small web frontend and serve that along the API.

_Note: This and the next chapter won't have much Rust code. Feel free to skip to the [final application](final-application.md) if you just want the code._

✅ Match `/` and `/app.js` and serve the respective files.
To simplify deployment you can embed the files directly into the binary using `include_str!`.

```rust
match (req.get_method(), req.get_path()) {
    (&Method::GET, "/") => Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(include_str!("index.html"))),

    (&Method::GET, "/app.js") => Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JAVASCRIPT)
        .with_body(include_str!("app.js"))),

    // (cut)
}
```

✅ Create a `src/index.html` file with a basic HTML structure.

```html
{{#include ../../../../crates/edge/src/index.html:1:7}}
{{#include ../../../../crates/edge/src/index.html:39:}}
```

✅ To upload a picture the frontend needs a file selector, so add the following in between the `<body>` tags.

```html
{{#include ../../../../crates/edge/src/index.html:8}}
```

✅ Additionally the user should be able to select a filter. List out all available ones manually.

```html
{{#include ../../../../crates/edge/src/index.html:9:34}}
```

✅ To show that an upload is in progress add a `<span>` where you can show a message.

```html
{{#include ../../../../crates/edge/src/index.html:35}}
```

✅ You also need a place to display the resulting image.

```html
{{#include ../../../../crates/edge/src/index.html:37}}
```

✅ And last but not least include the JavaScript frontend code.

```html
{{#include ../../../../crates/edge/src/index.html:38}}
```

---

The next chapter will guide you through writing the JavaScript frontend code.
