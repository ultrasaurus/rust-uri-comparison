
only `uriparse` requires `+nightly` but since these are all in the same crate, any example need to be run with `+nightly`

```
cargo +nightly run --example uriparse
cargo +nightly run --example rust-url
cargo +nightly run --example hyper-uri
```


* rust-url: `cargo add url`
  * https://github.com/servo/rust-url
  * https://docs.rs/url/1.7.2/url/
* uriparse: `cargo add uriparse`
  * https://docs.rs/uriparse/0.6.1/uriparse/uri/struct.URI.html
* hyper-uri: `cargo add hyper`
  * https://docs.rs/hyper/0.12.8/hyper/struct.Uri.html