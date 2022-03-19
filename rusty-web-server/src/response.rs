use crate::request::RawHttpRequest;
use std::io::Write;

pub struct HttpResponse;

impl HttpResponse {
    pub fn send<C: Into<String>>(request: RawHttpRequest, content: C) -> Result<(), String> {
        let RawHttpRequest { mut stream } = request;
        let response = Self::response(200, content.into());
        stream
            .write(response.as_bytes())
            .map_err(|e| e.to_string())?;
        stream.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn response<C: Into<String>>(status: u64, content: C) -> String {
        let content = content.into();
        let response = format!(
            "HTTP/1.1 {status} OK\r\nContent-Length: {}\r\n\r\n{content}",
            content.len(),
        );

        response
    }
}
