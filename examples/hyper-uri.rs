use hyper::http::Uri;

fn main() {
  let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
  assert_eq!(uri.path(), "/foo/bar");
  assert_eq!(uri.query(), Some("baz"));
  assert_eq!(uri.host(), None);

  let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
  assert_eq!(uri.scheme_part().map(|s| s.as_str()), Some("https"));
  assert_eq!(uri.host(), Some("www.rust-lang.org"));
  assert_eq!(uri.path(), "/install.html");
}
