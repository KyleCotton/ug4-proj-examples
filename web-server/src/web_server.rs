use rusty_junctions::Junction;
use std::net::{TcpListener, TcpStream};

use crate::request::Request;

const ADDRESS: &'static str = "localhost:8080";

enum MultiThreadedMode {
    SingleHandler,
    SeparateHandler,
}

pub struct WebServer {}

impl WebServer {
    pub fn start_with_single_handler() {
        WebServer::run_multi_threaded(MultiThreadedMode::SingleHandler);
    }

    pub fn start_with_separate_handler() {
        WebServer::run_multi_threaded(MultiThreadedMode::SeparateHandler);
    }

    pub fn start_single_threaded() {
        // Create a listener for the TCP traffic
        let listener = TcpListener::bind(ADDRESS).unwrap();
        let mut value = 0;
        listener.incoming().for_each(|stream| {
            if let Ok(mut request) = Request::from_tcp_stream(stream.unwrap()) {
                value = request.handle_now(value);
            }
        })
    }

    fn run_multi_threaded(mode: MultiThreadedMode) {
        // Create a listener for the TCP traffic
        let listener = TcpListener::bind(ADDRESS).unwrap();

        // Create a new junction
        let j = Junction::new();

        // Create the channel to support the server
        let connection = j.send_channel::<TcpStream>();
        let request = j.send_channel::<Request>();

        // Create the channel to facilitate the concurrent state, addition, and retrieval
        let add = j.send_channel::<i64>();
        let value = j.send_channel::<i64>();
        let get = j.recv_channel::<i64>();

        match mode {
            MultiThreadedMode::SingleHandler => {
                let a = add.clone();
                let g = get.clone();
                j.when(&connection).then_do(move |stream| {
                    if let Ok(mut request) = Request::from_tcp_stream(stream) {
                        println!("{:?}", request);
                        request.handle_pooled(&a, &g);
                    };
                });
            }
            MultiThreadedMode::SeparateHandler => {
                // Accept all of the connections, and add the request to the request queue
                let r = request.clone();
                j.when(&connection).then_do(move |stream| {
                    if let Ok(request) = Request::from_tcp_stream(stream) {
                        println!("{:?}", request);
                        r.send(request).expect("Add Request");
                    };
                });

                // Hander for all of the possible requests
                let a = add.clone();
                let g = get.clone();
                j.when(&request)
                    .then_do(move |mut request| request.handle_pooled(&a, &g));
            }
        };

        // When there is a vale in the storage and we are adding add the value to
        // the current one and send a new value.
        let v = value.clone();
        j.when(&value).and(&add).then_do(move |value, num| {
            std::thread::sleep(crate::DELAY_TIME);
            v.send(value + num).expect("Updating value");
        });

        // When there is a value and we want to get the value, send the same
        // value again, and return it.
        let v = value.clone();
        j.when(&value).and_recv(&get).then_do(move |value| {
            v.send(value).expect("Keep the same value");
            value
        });

        // Set the default value for the cell
        value.send(0).expect("Setting the default value");

        // Send all of the connection to the connection queue to be processed
        listener
            .incoming()
            .for_each(|stream| connection.send(stream.unwrap()).unwrap())
    }
}
