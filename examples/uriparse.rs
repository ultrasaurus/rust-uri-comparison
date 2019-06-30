use std::convert::TryFrom;
use uriparse::{Authority, Fragment, Path, Scheme, URI};

fn main() {
  let uri = URI::try_from("http://example.com:80/my/path").unwrap();
  assert_eq!(uri.authority().unwrap().to_string(), "example.com:80");

  let uri = URI::builder()
      .with_scheme(Scheme::HTTP)
      .with_authority(Some(Authority::try_from("example.com").unwrap()))
      .with_path(Path::try_from("/my/path").unwrap())
      .build()
      .unwrap();
  assert_eq!(uri.to_string(), "http://example.com/my/path");

  let uri = URI::from_parts(
      "http",
      Some("example.com"),
      "",
      Some("query"),
      None::<Fragment>
  ).unwrap();
  assert_eq!(uri.to_string(), "http://example.com/?query");
}
