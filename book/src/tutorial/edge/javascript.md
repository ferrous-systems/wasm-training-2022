# JavaScript

In the previous chapter you already created a handler in your application returning an `app.js` file and also referenced that in your HTML code.
Time to write this now.

The plan is to:

* load the image data from the selected file
* post this image data with the selected filter name to the backend
* display the resulting file on the web page

✅ If there's a change on the file selector ("the user selected a file") or a new filter is selected you should send the image to the backend.

```javascript
{{#include ../../../../crates/edge/src/app.js:47:}}
```

✅ The above calls a new function.

```javascript
{{#include ../../../../crates/edge/src/app.js:1}}
  // (to be filled in)
}
```

✅ First grab the selected file and let the user know the application is working.


```javascript
{{#include ../../../../crates/edge/src/app.js:2:8}}
```

✅ Start by displaying the image.
The JavaScript web API lets you turn the file object into an object URL that can be displayed.

```javascript
{{#include ../../../../crates/edge/src/app.js:10:12}}
```

✅ Next fetch the selected image filter name. If it's `none` you don't need to do any work!

```javascript
{{#include ../../../../crates/edge/src/app.js:14:18}}
```

✅ Reading the file to then submit it requires some additional web APIs.
A `FileReader` allows to read the selected file.

```javascript
{{#include ../../../../crates/edge/src/app.js:20:21}}
    // (to be filled in)
{{#include ../../../../crates/edge/src/app.js:40:42}}
```

✅ Within the `FileReader`'s `onload` callback create a `POST` request using the `fetch` API.
The response can be turned back into an object URL, that you can then display again as before.

```javascript
{{#include ../../../../crates/edge/src/app.js:22:39}}
```

---

And that's it for the frontend.
Next you can run the full application locally.
