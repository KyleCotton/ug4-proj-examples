mod header;
mod request;
mod web_server;

use crate::web_server::WebServer;

use std::time::Duration;
const DELAY_TIME: Duration = Duration::from_millis(20);

fn main() {
    if cfg!(feature = "single_threaded") {
        println!("---> Server Starting: Single Threaded");
        let _web_server = WebServer::start_single_threaded();
    }

    if cfg!(feature = "single_handler") {
        println!("---> Server Starting: Single Handler");
        let _web_server = WebServer::start_with_single_handler();
    }

    if cfg!(feature = "separate_handler") {
        println!("---> Server Starting: Separate Handler");
        let _web_server = WebServer::start_with_separate_handler();
    }
}
