use crate::{
    action::{Action, Operation},
    request::{ParsedHttpRequest, RawHttpRequest},
    response::HttpResponse,
    WebServer, ADDRESS,
};
use rusty_junctions::Junction;
use std::{cmp::Ordering, net::TcpListener};

#[derive(Clone)]
pub struct Metrics {
    pub increment_value: i64,
    pub decrement_value: i64,
}

pub struct OriginalServer;
impl WebServer for OriginalServer {
    fn run() {
        let j = Junction::new();

        let value = j.send_channel::<i64>();
        let metrics = j.send_channel::<Metrics>();

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

        let metrics_clone = metrics.clone();
        let value_clone = value.clone();
        j.when(&increment).and(&metrics).and(&value).then_do(
            move |(i, strm),
                  Metrics {
                      increment_value,
                      decrement_value,
                  },
                  val| {
                let message = format!("New Value: {val}\nIncrements: {increment_value}");
                value_clone.send(i + val).unwrap();
                metrics_clone
                    .send(Metrics {
                        increment_value: increment_value + 1,
                        decrement_value,
                    })
                    .unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            },
        );

        let metrics_clone = metrics.clone();
        let value_clone = value.clone();
        j.when(&decrement).and(&metrics).and(&value).then_do(
            move |(d, strm),
                  Metrics {
                      increment_value,
                      decrement_value,
                  },
                  val| {
                let message = format!("New Value: {val}\nDecrements: {decrement_value}");
                value_clone.send(d + val).unwrap();
                metrics_clone
                    .send(Metrics {
                        decrement_value: decrement_value + 1,
                        increment_value,
                    })
                    .unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            },
        );

        let metrics_clone = metrics.clone();
        let value_clone = value.clone();
        j.when(&get)
            .and(&value)
            .and(&metrics)
            .then_do(move |strm, val, metrics| {
                let message = format!(
                    "Current Value: {val}\nIncrements: {}, Decrements: {}",
                    metrics.increment_value, metrics.decrement_value
                );
                metrics_clone.send(metrics).unwrap();
                value_clone.send(val).unwrap();
                HttpResponse::send(strm, message)
                    .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                    .ok();
                std::thread::sleep(std::time::Duration::from_millis(10));
            });

        metrics
            .send(Metrics {
                increment_value: 0,
                decrement_value: 0,
            })
            .unwrap();
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
