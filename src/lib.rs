#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_const_for_fn
)]
#![deny(unsafe_code, unreachable_pub)]

mod actors;
pub mod client;
pub mod codec;
pub mod conn;
pub mod payload;
mod pool;

pub use pool::SerdeProcessingError;
