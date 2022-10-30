# JavaScript

JavaScript is used to handle events from the HTML form
and pass the data over to the WebAssembly module,
which first needs to be loaded and instantiated of course.

✅ Start by creating an empty `app/app.js` file.
This is where all the code will go now.

✅ You already have the JavaScript shim and the wasm file ready to go, so you can start by importing it.

```javascript
{{#include ../../../../crates/web/app/app.js:1}}
```

✅ You need to call and await `init` to actually load, compile and instantiate the WebAssembly module.
Once you have done that imported `apply_filter` will be a function you can call.

```javascript
{{#include ../../../../crates/web/app/app.js:3}}
```

✅ Whenever the user selects an image or changes the filter you should load the image and apply the filter. Hook up the `onchange` events of both the file input and the select now.

```javascript
{{#include ../../../../crates/web/app/app.js:5:10}}
```

`imageFilter` will be the function that handles all the logic.

✅ But first add a small helper `typedArrayToURL`.
JavaScript's [`TypedArray`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray) is an array-like view of a binary data buffer.
Your converted image will be in such a buffer.
To display that in the browser you need to turn it into an object url.
It's enough to only handle the PNG image format.

```javascript
{{#include ../../../../crates/web/app/app.js:12:16}}
```

✅ Now start writing your `imageFilter` function.

```javascript
{{#include ../../../../crates/web/app/app.js:18}}
  // (to be filled in)
}
```

✅ You should start by checking for the file the user selected.

```javascript
{{#include ../../../../crates/web/app/app.js:19:23}}
```

✅ This is a good time to let the user now that the application is working.
You can also display the original image without a filter applied yet.

```javascript
{{#include ../../../../crates/web/app/app.js:24:29}}
```

✅ Next fetch the selected image filter name. If it's "none" you don't need to do any work!

```javascript
{{#include ../../../../crates/web/app/app.js:31:35}}
```

✅ Reading the file to then pass it to your WebAssembly function requires to read it and turn it into an array buffer first. That's available directly on the file object you already have.

```javascript
{{#include ../../../../crates/web/app/app.js:37}}
```

✅ The `apply_filter` function expects an array of `u8` and the filter name as a string.
To get that array from our `img` you can call [`new Uint8Array`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array), passing your image data.
A string is automatically handled by the `wasm-bindgen` shim.

```javascript
{{#include ../../../../crates/web/app/app.js:38}}
```

And that is all you need to call a function in the WebAssembly module already.

✅ What's left to do is turn the image into a blob URL you can display and inform the user that the work is done.

```javascript
{{#include ../../../../crates/web/app/app.js:39:42}}
```

---

The [next chapter](run-locally.md) will tell you again how to build and run the application locally.
