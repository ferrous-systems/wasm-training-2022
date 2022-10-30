# JavaScript

In the previous chapter you already created a handler in your application returning an `app.js` file and also referenced that in your HTML code.
Time to write the JavaScript code now.

The plan is to:

* load the image data from the selected file
* post this image data with the selected filter name to the backend
* display the resulting file on the web page

_Note: A lot of this JavaScript code is similar to the one from the [Web tutorial](../web.md). The important difference is in the last step where instead of calling into WebAssembly you send the image to a server._

✅ If there's a change on the file selector ("the user selected a file") or a new filter is selected you should send the image to the backend.

```javascript
{{#include ../../../../crates/edge/src/app.js:1:6}}
```

✅ The above calls a new function.

```javascript
{{#include ../../../../crates/edge/src/app.js:8}}
  // (to be filled in)
}
```

✅ First grab the selected file and let the user know the application is working.


```javascript
{{#include ../../../../crates/edge/src/app.js:9:15}}
```

✅ Start by displaying the image.
The JavaScript web API lets you turn the file object into an object URL that can be displayed.

```javascript
{{#include ../../../../crates/edge/src/app.js:17:19}}
```

✅ Next fetch the selected image filter name. If it's `none` you don't need to do any work!

```javascript
{{#include ../../../../crates/edge/src/app.js:14:18}}
```

✅ Reading the file to then submit it requires to read it and turn it into an array buffer first. That's available directly on the file object you already have.

```javascript
{{#include ../../../../crates/edge/src/app.js:27}}
```

✅ Now you can create a `POST` request to the `/image` API endpoint using the `fetch` API, using the image data as the body.

```javascript
{{#include ../../../../crates/edge/src/app.js:28:36}}
```

✅ The response can be turned back into an object URL, that you can then display again as before.

```javascript
{{#include ../../../../crates/edge/src/app.js:38:40}}
```

---

And that's it for the frontend.
[Next](run-locally.md) you can run the full application locally.
