#![feature(result_flattening)]

mod action;
mod request;
mod response;
mod server;

const ADDRESS: &'static str = "localhost:8080";

#[cfg(feature = "single_threaded")]
pub use crate::server::SingleThreadedWebServer as Server;

#[cfg(feature = "original")]
pub use crate::server::OriginalServer as Server;

#[cfg(feature = "standard")]
pub use crate::server::StandardServer as Server;

#[cfg(feature = "macro")]
pub use crate::server::MacroServer as Server;

pub trait WebServer {
    fn run();
}
