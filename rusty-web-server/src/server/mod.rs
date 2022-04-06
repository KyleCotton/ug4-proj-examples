#[cfg(feature = "single_threaded")]
pub mod single_threaded_server;
#[cfg(feature = "single_threaded")]
pub use single_threaded_server::SingleThreadedWebServer;

#[cfg(feature = "original")]
pub mod original_server;
#[cfg(feature = "original")]
pub use original_server::OriginalServer;

#[cfg(feature = "standard")]
pub mod standard_server;
#[cfg(feature = "standard")]
pub use standard_server::StandardServer;

#[cfg(feature = "macro")]
pub mod macro_server;
#[cfg(feature = "macro")]
pub use macro_server::MacroServer;
