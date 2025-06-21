#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    // missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    // clippy::missing_panics_doc,
    clippy::missing_const_for_fn
)]
#![deny(unsafe_code, unreachable_pub)]
pub mod client;
pub mod config;
pub mod payload;

mod actors;
mod codec;
mod conn;
mod oauth2;
mod pool;

pub use conn::ConnectionError;
pub use pool::SerdeProcessingError;
