use crate::header::Header;
use rusty_junctions::channels::{RecvChannel, SendChannel};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub(crate) path: String,
    pub(crate) headers: Vec<Header>,
    pub(crate) stream: TcpStream,
}

impl Request {
    pub fn from_tcp_stream(mut stream: TcpStream) -> Self {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let (path, headers) = Header::from_request(&buffer);

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

    pub fn handle(&mut self, add: &SendChannel<i64>, get: &RecvChannel<i64>) {
        let path = self.clone().path;

        let response = match get_path_and_value(path) {
            (Some(p), Some(v)) if p.as_str() == "add" => {
                add.send(v).expect("Queue number");
                format!("Adding {} to the adding queue", v)
            }
            (Some(p), _) if p.as_str() == "get" => {
                let value = get.recv().expect("Getting Value");
                format!("Value retrived: {}", value)
            }
            _ => "Invalid Request".to_string(),
        };

        self.respond(&*response);
    }
}

fn get_path_and_value(path: String) -> (Option<String>, Option<i64>) {
    let values: Vec<&str> = path.split("/").collect();
    let name = values.get(1).map(|s| s.to_string());
    let value = match values.get(2).map(|s| s.parse::<i64>()) {
        Some(Ok(n)) => Some(n),
        _ => None,
    };

    (name, value)
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
