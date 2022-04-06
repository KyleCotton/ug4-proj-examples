use crate::{
    action::{Action, Operation},
    request::{ParsedHttpRequest, RawHttpRequest},
    response::HttpResponse,
    WebServer, ADDRESS,
};
use rusty_junctions::Junction;
use std::{cmp::Ordering, net::TcpListener};

pub struct StandardServer;
impl WebServer for StandardServer {
    fn run() {
        let j = Junction::new();

        let value = j.send_channel::<i64>();
        let increment_value = j.send_channel::<i64>();
        let decrement_value = j.send_channel::<i64>();

        let raw_connection = j.send_channel::<RawHttpRequest>();
        let action = j.send_channel::<Action>();
        let increment = j.send_channel::<(i64, RawHttpRequest)>();
        let decrement = j.send_channel::<(i64, RawHttpRequest)>();
        let get = j.send_channel::<RawHttpRequest>();

        // Parse the HTTP Request into an Action then dispatch to the Action Handler
        let action_clone = action.clone();
        j.when(&raw_connection).then_do(move |raw_request| {
            let request = ParsedHttpRequest::from_raw_http_request(raw_request).unwrap();
            if let Ok(action) = Action::from_parsed_http_request(request) {
                action_clone.send(action).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        });

        let decrement_clone = decrement.clone();
        let increment_clone = increment.clone();
        let get_clone = get.clone();
        j.when(&action)
            .then_do(move |Action { stream, operation }| {
                match operation {
                    Operation::Add(v) => {
                        match 0.cmp(&v) {
                            Ordering::Greater => decrement_clone.send((v, stream)).unwrap(),
                            Ordering::Less | Ordering::Equal => {
                                increment_clone.send((v, stream)).unwrap()
                            }
                        };
                    }
                    Operation::Get => get_clone.send(stream).unwrap(),
                };
                std::thread::sleep(std::time::Duration::from_millis(10));
            });

        let increment_value_clone = increment_value.clone();
        let value_clone = value.clone();
        j.when(&increment)
            .and(&increment_value)
            .and(&value)
            .then_do(move |(i, strm), inc, val| {
                let message = format!("New Value: {val}\nIncrements: {inc}");
                value_clone.send(i + val).unwrap();
                increment_value_clone.send(inc + 1).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            });

        let decrement_value_clone = decrement_value.clone();
        let value_clone = value.clone();
        j.when(&decrement)
            .and(&decrement_value)
            .and(&value)
            .then_do(move |(d, strm), dec, val| {
                let message = format!("New Value: {val}\nDecrements: {dec}");
                value_clone.send(d + val).unwrap();
                decrement_value_clone.send(dec + 1).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            });

        let increment_value_clone = increment_value.clone();
        let decrement_value_clone = decrement_value.clone();
        let value_clone = value.clone();
        j.when(&get)
            .and(&value)
            .and(&increment_value)
            .and(&decrement_value)
            .then_do(move |strm, val, inc, dec| {
                let message = format!("Current Value: {val}\nIncrements: {inc}, Decrements: {dec}");
                increment_value_clone.send(inc).unwrap();
                decrement_value_clone.send(dec).unwrap();
                value_clone.send(val).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            });

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
