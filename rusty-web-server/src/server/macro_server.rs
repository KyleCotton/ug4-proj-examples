use crate::{
    action::{Action, Operation},
    request::{ParsedHttpRequest, RawHttpRequest},
    response::HttpResponse,
    WebServer, ADDRESS,
};
use rusty_junctions::junction;
use std::{cmp::Ordering, net::TcpListener};

pub struct MacroServer;
impl WebServer for MacroServer {
    fn run() {
        junction! {
            value as Send::i64,
            increment_value as Send::i64,
            decrement_value as Send::i64,

            raw_connection as Send::RawHttpRequest,
            action as Send::Action,
            increment as Send::(i64, RawHttpRequest),
            decrement as Send::(i64, RawHttpRequest),
            get as Send::RawHttpRequest,

            |raw_connection| {
                let request = ParsedHttpRequest::from_raw_http_request(raw_connection).unwrap();
                if let Ok(action) = Action::from_parsed_http_request(request) {
                    action_super.send(action).unwrap();
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            },

            |action| {
                let Action {stream, operation} = action;
                match operation {
                    Operation::Add(v) => {
                        match 0.cmp(&v) {
                            Ordering::Greater => decrement_super.send((v, stream)).unwrap(),
                            Ordering::Less | Ordering::Equal => {
                                increment_super.send((v, stream)).unwrap()
                            }
                        };
                    }
                    Operation::Get => get_super.send(stream).unwrap(),
                };
                std::thread::sleep(std::time::Duration::from_millis(10));
            },

            |increment, increment_value, value| {
                let (i, strm) = increment;
                let message = format!("New Value: {value}\nIncrements: {increment_value}");
                value_super.send(i + value).unwrap();
                increment_value_super.send(increment_value + 1).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            },

            |decrement, decrement_value, value| {
                let (d, strm) = decrement;
                let message = format!("New Value: {value}\nDecrements: {decrement_value}");
                value_super.send(d + value).unwrap();
                decrement_value_super.send(decrement_value + 1).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            },

            |get, value, increment_value, decrement_value| {
                let message = format!("Current Value: {value}\nIncrements: {increment_value}, Decrements: {decrement_value}");
                increment_value_super.send(increment_value).unwrap();
                decrement_value_super.send(decrement_value).unwrap();
                value_super.send(value).unwrap();
                HttpResponse::send(get, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        };

        increment_value.send(0).unwrap();
        decrement_value.send(0).unwrap();
        value.send(0).unwrap();

        TcpListener::bind(ADDRESS)
            .expect("Failed to start TcpListener")
            .incoming()
            .for_each(|strm| {
                let raw_request = RawHttpRequest::new(strm.unwrap());
                raw_connection.send(raw_request).unwrap();
            });
    }
}
