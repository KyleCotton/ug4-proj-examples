use httparse::Request as HttpRequest;

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn from_request(request: &[u8]) -> (String, Vec<Header>) {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = HttpRequest::new(&mut headers);
        req.parse(request).unwrap();

        let headers = req
            .headers
            .to_vec()
            .iter()
            .map(|h| Header {
                name: String::from(h.name),
                value: String::from_utf8(h.value.to_vec()).unwrap(),
            })
            .collect();

        let path = String::from(req.path.unwrap());

        (path, headers)
    }
}
