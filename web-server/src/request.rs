use httparse::Request as HttpRequest;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub(crate) path: String,
    pub(crate) headers: Vec<Header>,
    pub(crate) stream: TcpStream,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Clone for Request {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            headers: self.headers.clone(),
            stream: self.stream.try_clone().expect("Failed to clone TCP Stream"),
        }
    }
}

impl Request {
    pub fn from_tcp_stream(mut stream: TcpStream) -> Self {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // Parse the buffer into a HttpRequest
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = HttpRequest::new(&mut headers);
        req.parse(&buffer).unwrap();

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

        Self {
            stream: stream,
            headers,
            path,
        }
    }

    pub fn respond<'s, T>(&mut self, content: T)
    where
        T: Into<&'s str>,
    {
        let content = content.into();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
        // println!("Response: {}", response);
    }
}
