# HTML Frontend

TODO: minimal web frontend

✅ Create a `app/index.html` file with a basic HTML structure.

```html
{{#include ../../../../crates/web/app/index.html:1:7}}
{{#include ../../../../crates/web/app/index.html:39:}}
```

✅ To upload a picture the frontend needs a file selector, so add the following in between the `<body>` tags.

```html
{{#include ../../../../crates/web/app/index.html:8}}
```

✅ Additionally the user should be able to select a filter. List out all available ones manually.

```html
{{#include ../../../../crates/web/app/index.html:9:34}}
```

✅ To show that an upload is in progress add a `<span>` where you can show a message.

```html
{{#include ../../../../crates/web/app/index.html:35}}
```

✅ You also need a place to display the resulting image.

```html
{{#include ../../../../crates/web/app/index.html:37}}
```

✅ And last but not least include the JavaScript frontend code.

```html
{{#include ../../../../crates/web/app/index.html:38}}
```
