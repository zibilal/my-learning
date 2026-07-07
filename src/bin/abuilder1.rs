#[derive(Debug)]
struct RequestBuilder {
    url: String,
    method: String,
}
impl RequestBuilder {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
        }
    }
    fn method(self, method: &str) -> Self {
        Self { method: method.to_string(), ..self }
    }
}

fn main() {
    let req = RequestBuilder::new("https://www.rust-lang.org/")
        .method("GET").method("POST");
    println!("{:?}", req);
}