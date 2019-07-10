
only `uriparse` requires `+nightly` but since these are all in the same crate, any example need to be run with `+nightly`

```
cargo +nightly run --example uriparse
cargo +nightly run --example rust-url
cargo +nightly run --example hyper-uri
```


* rust-url: [WHATWG URL standard](https://url.spec.whatwg.org/)
  * `cargo add url`
  * https://docs.rs/url/1.7.2/url/
  * https://github.com/servo/rust-url
* uriparse: [IETF URI RFC3986](https://tools.ietf.org/html/rfc3986)
  * `cargo add uriparse`
  * https://docs.rs/uriparse/0.6.1/uriparse/uri/struct.URI.html
  * https://github.com/sgodwincs/uriparse-rs
* hyper-uri: `request-target` as defined in [IETF HTTP 1.1 Specification - RFC7230 Section 5.3](https://tools.ietf.org/html/rfc7230#section-5.3)
  * `cargo add hyper`
  * https://docs.rs/hyper/0.12.8/hyper/struct.Uri.html
  * https://github.com/hyperium/hyper