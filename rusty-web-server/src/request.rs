use httparse::Request as ParseRequest;
use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub struct RawHttpRequest {
    pub(crate) stream: TcpStream,
}

impl Clone for RawHttpRequest {
    fn clone(&self) -> Self {
        let stream = self.stream.try_clone().expect("Failed to clone Stream");
        Self { stream }
    }
}

impl RawHttpRequest {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

pub struct ParsedHttpRequest {
    pub stream: RawHttpRequest,
    pub path: String,
}

impl ParsedHttpRequest {
    pub fn from_raw_http_request(mut raw_http_request: RawHttpRequest) -> Result<Self, String> {
        let mut buffer = [0; 1024];
        raw_http_request
            .stream
            .read(&mut buffer)
            .map_err(|e| e.to_string())?;

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = ParseRequest::new(&mut headers);
        req.parse(&buffer).map_err(|_| "Failed to parse Request")?;

        let path = req
            .path
            .ok_or("Request has no path".to_string())?
            .to_string();

        Ok(Self {
            path,
            stream: raw_http_request,
        })
    }
}
