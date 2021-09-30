use rusty_junctions::{
    channels::{RecvChannel, SendChannel},
    Junction,
};
use std::net::{TcpListener, TcpStream};

use request::Request;
mod request;

const ADDRESS: &'static str = "localhost:8080";

fn main() {
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

    // Accept all of the connections, and add the request to the request
    // queue
    let r = request.clone();
    j.when(&connection).then_do(move |stream| {
        let request = Request::from_tcp_stream(stream);
        println!("{:?}", request);
        r.send(request).expect("Add Request");
    });

    // Hander for all of the possible requests
    let a = add.clone();
    let g = get.clone();
    j.when(&request)
        .then_do(move |request| handle_request(request, &a, &g));

    // When there is a vale in the storage and we are adding add the value to
    // the current one and send a new value.
    let v = value.clone();
    j.when(&value)
        .and(&add)
        .then_do(move |value, num| v.send(value + num).expect("Updating value"));

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
    println!("---> Server Listening on: {}", ADDRESS);
    listener
        .incoming()
        .for_each(|stream| connection.send(stream.unwrap()).unwrap())
}

fn handle_request(mut request: Request, add: &SendChannel<i64>, get: &RecvChannel<i64>) {
    let path = request.clone().path;

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

    request.respond(&*response);
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
