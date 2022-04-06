use crate::{
    action::{Action, Operation},
    request::{ParsedHttpRequest, RawHttpRequest},
    response::HttpResponse,
    WebServer, ADDRESS,
};
use std::{cmp::Ordering, net::TcpListener};

pub struct SingleThreadedWebServer;
impl WebServer for SingleThreadedWebServer {
    fn run() {
        let mut value = 0;
        let mut number_of_increments = 0;
        let mut number_of_decrements = 0;
        let mut times_passing_zero = 0;

        for strm in TcpListener::bind(ADDRESS)
            .expect("Failed to start TcpListener")
            .incoming()
        {
            std::thread::sleep(5 * std::time::Duration::from_millis(10));
            if let Err(e) = strm {
                println!("Stream Error: {:?}", e.to_string());
                continue;
            }
            let strm = strm.unwrap();

            let raw_request = RawHttpRequest::new(strm);
            let request = ParsedHttpRequest::from_raw_http_request(raw_request);
            if let Err(e) = request {
                println!("Request Error: {:?}", e.to_string());
                continue;
            }
            let request = request.unwrap();

            let action = Action::from_parsed_http_request(request);
            if let Err(e) = action {
                println!("Action Error: {:?}", e.to_string());
                continue;
            }
            let Action { stream, operation } = action.unwrap();

            let response = match operation {
                Operation::Add(v) => {
                    let mut message;
                    match 0.cmp(&v) {
                        Ordering::Greater => {
                            number_of_decrements += 1;
                            message = format!("Decrements: {number_of_decrements}");
                        },
                        Ordering::Less => {
                            number_of_increments += 1;
                            message = format!("Increments: {number_of_increments}");

                        },
                        _equal => message = String::new(),
                    };

                    if (value < 0 && value + v > 0) || (value > 0 && value + v < 0) {
                        times_passing_zero += 1;
                        message = format!("{message}, Times Passing Zero: {times_passing_zero}");
                    }

                    value += v;
                    format!("New Value: {value}\n{message}")
                }
                Operation::Get => format!("Current Value: {value}\nIncrements: {number_of_increments}, Decrements: {number_of_decrements}, Times Passing Zero: {times_passing_zero}"),
            };

            HttpResponse::send(stream, response)
                .map_err(|e| print!("HttpResponse Error: {}", e.to_string()))
                .ok();
        }
    }
}
