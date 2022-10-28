# Use cases everywhere else

WebAssembly is also supported outside of the browser environment.
There it can be used for a wide variety of applications,
making use of its sandboxing and security functionality.

Some possible use cases include:

## Plugins

User-facing native applications can safely support user-contributed plugins.
These plugins are compiled to WebAssembly modules and the application can run them in a restricted environment within the application, allowing access to only a small part of the application.

## Serverless

"Serverless" can describe a wide variety of concepts.
In recent times it became known as a cloud computing execution model,
where cloud providers allocate machine resources on demand, managing it for their users
and executing the user's application code on request.

Most commonly this is offered as a Function as a Service (FaaS) platform,
where small application logic is executed on incoming requests, using limited resources (CPU, time, memory).

WebAssembly allows that users can write this logic in a language of their choice
and the provider supports a general WebAssembly execution environment, often accompanied with a provider-specific SDK.
The provider can leverage the WebAssembly sandbox mechanism to provide per-request isolation & performance.

We look at one of these serverless offerings:
[Fastly's Compute@Edge](compute-at-edge.md)

## Docker

Docker recently announced a [Technical Preview of their WebAssembly support](https://www.docker.com/blog/docker-wasm-technical-preview/).
Docker containers can be used to build and distribute WebAssembly applications.
The Docker engine can then extract and run this WebAssembly application in a wasm runtime,
all while using the familiar Docker tooling.

## Third-party library sandboxing

[RLBox](https://rlbox.dev/) is a toolkit for sandboxing third party C libraries.
This allows to run third-party libraries within an existing application,
but restricting the access to only what is directly provided to the library as input.
Thus reducing the attack surface of this part of the code.
It is in use in Mozilla Firefox.
