pub struct Header {
  method: String,
  accept: String,
  url: String
}

impl Header {
  pub fn new<S: Into<String>>(method: S, accept: S, url: S) -> Header {
    let method = method.into();
    let accept = accept.into();
    let url = url.into();
    Header { method: method, accept: accept, url: url }
  }  

  pub fn build(&self) -> String {
    format!("{} / HTTP/1.0\r\n Host: {}\r\n\r\n Accept: {}\r\n\r\n", self.method, self.url, self.accept)
  }
}

#[test]
fn it_should_not_panic() {
  let test = Header::new("GET", "application/json", "https://skimdb.npmjs.com/registry/_changes");
  test.build();
}
