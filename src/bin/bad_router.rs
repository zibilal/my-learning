use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}


struct BasicRouter<C>
where
   C: Fn(&Request) -> Response,
{
    routes: HashMap<String, C>
}


fn main() {}