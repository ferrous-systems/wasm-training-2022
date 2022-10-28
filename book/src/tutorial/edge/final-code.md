# Final application

To recap your final Rust code should look something like this:

```rust
{{#include ../../../../crates/edge/src/main.rs}}
```

The frontend in HTML:

```html
{{#include ../../../../crates/edge/src/index.html}}
```

And the JavaScript frontend code:

```javascript
{{#include ../../../../crates/edge/src/app.js}}
```

You can build and serve your application locally like this:

```
fastly compute serve
```

---

Some ideas on what to do next:

* Can you return different image formats? Different sizes?
* What other task could be suitable for edge computing?
