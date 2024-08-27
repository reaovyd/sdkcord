//! A library that provides the functionality for IPC communication with Discord
//!
//! # Overview
//! `ipccord` is an interface library to communicate with the Discord client running on your
//! machine.
//!
//! # Objectives
//! - This is currently only to be used with the underlying application since the goal currently is
//!   to really just get a way for people to have whatever rich presence they want in Discord.
//! - In a possible future, we could extend the functionality to support whatever is mentioned in the
//!   [RPC docs][rpcdocs] since I think
//!
//! ## Blocking
//! We'll currently only support async calls and the executor will be Tokio, so there will be no
//! blocking support at the moment.
//!
//! ## Platforms supported
//! The major platforms (Linux, Windows, MacOS) will try to be supported.
//!
//! [rpcdocs]: https://discord.com/developers/docs/topics/rpc
#![warn(
    missing_debug_implementations,
    // missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_const_for_fn
)]
#![deny(unsafe_code, unreachable_pub)]

mod codec;
pub mod opcode;
