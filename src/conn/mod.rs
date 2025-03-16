#[cfg(unix)]
pub(crate) mod unix;

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(windows)]
pub use windows::*;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ConnectionError {
    #[error(
        "failed to connect after trying all possible paths! if you think the IPC path on your system lives elsewhere, please open an issue on the repository!"
    )]
    ConnectionFailed,
}
