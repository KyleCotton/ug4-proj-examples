use httparse::Request as HttpRequest;

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn from_request(request: &[u8]) -> Result<(String, Vec<Header>), String> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = HttpRequest::new(&mut headers);
        req.parse(request).map_err(|_| "Failed to parse Request")?;

        let mut headers = Vec::new();

        for h in req.headers.to_vec() {
            let name = String::from(h.name);
            let header_value = String::from_utf8(h.value.to_vec());

            match header_value {
                Ok(value) => headers.push(Header { name, value }),
                _ => return Err("Failed to Parse Headers".to_string()),
            };
        }

        let path = match req.path {
            Some(p) => String::from(p),
            _ => return Err("Failed to Parse Path".to_string()),
        };

        Ok((path, headers))
    }
}
