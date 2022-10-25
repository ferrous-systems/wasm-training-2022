# Edge

_You can find the scaffolding for this example in `crates/edge`.
The finished example code is available in the `finished` branch._

In this tutorial you'll get familiar with:

* building for Fastly's Compute@Edge platform
* Handling and responding to a web request

We build a web API that applies a given filter to an image posted to it.
It returns the produced image over HTTP.
Additionally we also serve a bare-bones HTML form that allows us to use this API.
