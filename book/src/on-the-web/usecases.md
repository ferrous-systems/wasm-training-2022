# Use cases

WebAssembly originated as a successor to asm.js, a low-level subset of JavaScript,
and Google Native Client (NaCl), a technology to run a subset of native code in a sandboxed environment within the browser.
However WebAssembly in its current form is not tied to web.

WebAssembly on the web allows to build existing software written in a variety of languages
and run them as part of ordinary web applications.

The following is a list of interesting existing web applications using WebAssembly.

## [Pyodide](https://pyodide.org/)

A full Python distribution running in your browser.
It comes with builtin packages as well as support to install pure-Python packages from PyPi.

An in-browser REPL is available at <https://pyodide.org/en/stable/console.html>.

## [Datasette Lite](https://lite.datasette.io/)

Datasette is an open source multi-tool for exploring and publishing data.
It provides an interface to SQLite databases.

Datasette Lite is based on Pyodide and brings the full application to the browser.
You can open remote database and CSV files, executes queries and browse through the loaded database.

## [squoosh.app](https://squoosh.app/)

squoosh is an image compression web app, fully client-side.
It provides an interactive interface to resize an image and supports different output codecs.
Everything is happening client-side and images never leave the browser.

Source code is available on [GitHub](https://github.com/GoogleChromeLabs/squoosh).

## [Photoshop on the Web](https://creativecloud.adobe.com/cc/photoshop)

Photoshop on the Web is the nearly-complete Photoshop experience running in the browser.
It's currently in beta and not yet fully supported in all browsers.

## [Tailscale SSH Console](https://tailscale.com/blog/ssh-console/)

Tailscale is a VPN service that allows you to make your devices accessible within an overlay network, no matter where those devices are physically located.
It recently started to support SSH over their service with next to no setup.
They now offer an SSH console directly in the browser.
Their VPN client and networking code has been compiled to WebAssembly
and (encrypted) traffic goes directly to relay servers, but no additional proxies.
