use crate::header::Header;
use rusty_junctions::channels::{RecvChannel, SendChannel};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub(crate) path: String,
    // pub(crate) headers: Vec<Header>,
    pub(crate) stream: TcpStream,
}


impl Clone for Request {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            // headers: self.headers.clone(),
            stream: self.stream.try_clone().expect("Failed to clone TCP Stream"),
        }
    }
}

impl Request {
    pub fn from_tcp_stream(mut stream: TcpStream) -> Result<Self, String> {
        println!("---> New Connection");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        match Header::from_request(&buffer) {
            Ok((path, headers)) => Ok(Self {
                stream,
                // headers,
                path,
            }),
            Err(e) => {
                let mut request = Self {
                    stream,
                    // headers: Vec::new(),
                    path: String::from(""),
                };
                request.respond_err(&*e.to_string());
                return Err(e);
            }
        }
    }

    pub fn respond_err<'s, T>(&mut self, content: T)
    where
        T: Into<String>,
    {
        let content = content.into();
        let response = format!(
            "HTTP/1.1 500 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn respond_ok<'s, T>(&mut self, content: T)
    where
        T: Into<String>,
    {
        let content = content.into();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn handle_pooled(&mut self, add: &SendChannel<i64>, get: &RecvChannel<i64>) {
        std::thread::sleep(crate::DELAY_TIME);

        let path = self.clone().path;
        match handle_connection(path) {
            Action::Add(v) => {
                add.send(v).expect("Queue number");
                self.respond_ok(format!("Adding {} to the adding queue", v))
            }
            Action::Get => {
                let value = get.recv().expect("Getting Value");
                self.respond_ok(format!("Value retrived: {}", value))
            }
            _ => self.respond_err("Invalid Request"),
        }
    }

    pub fn handle_now(&mut self, value: i64) -> i64 {
        std::thread::sleep(crate::DELAY_TIME);

        let path = self.clone().path;
        match handle_connection(path) {
            Action::Add(v) => {
                self.respond_ok(format!("Adding {} to the adding queue", v));
                 value + v
            },
            Action::Get => {
                self.respond_ok(format!("Value retrived: {}", value));
                value
            },
            Action::Invalid => {
                self.respond_err("Invalid Request");
                value
            },
        }
    }
}

pub enum Action {
    Add(i64),
    Get,
    Invalid,
}

fn handle_connection(path: String) -> Action {
    let values: Vec<&str> = path.split("/").collect();
    let name = values.get(1).map(|s| s.to_string());
    let value = match values.get(2).map(|s| s.parse::<i64>()) {
        Some(Ok(n)) => Some(n),
        _ => None,
    };

    match (name, value) {
        (Some(p), Some(v)) if p.as_str() == "add" => Action::Add(v),
        (Some(p), _) if p.as_str() == "get" => Action::Get,
        _ => Action::Invalid,
    }
}
