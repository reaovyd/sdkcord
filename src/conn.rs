//! Module that provides functionality to handle the IPC connection.

use thiserror::Error;
use tokio::{io::AsyncWriteExt, net::UnixStream};

#[derive(Debug)]
pub struct Connection {
    inner: UnixStream,
}

impl Connection {
    const PIPE_ID_MIN: usize = 0;
    const PIPE_ID_MAX: usize = 10;

    /// Connects to the IPC stream and returns a new `Connection` instance.
    ///
    /// # Errors
    /// [Error] is returned if the connection to the IPC stream fails.
    pub async fn connect() -> Result<Self, Error> {
        let inner = Self::get_connected_stream().await?;
        Ok(Self { inner })
    }

    /// Automatically tries to find the IPC named pipe based on the `target_family`
    ///
    /// # Unix
    /// On Unix systems, the following directories are checked in order to find the IPC named pipe:
    /// - `$XDG_RUNTIME_DIR`
    /// - `$TMPDIR`
    /// - `$TMP`
    /// - `$TEMP`
    /// - `/tmp`
    ///
    /// # Windows
    /// On Windows systems, it must only be one file: \\.\pipe\discord-ipc-0
    async fn get_connected_stream() -> Result<UnixStream, Error> {
        for path in PATH_DIRS.into_iter().flatten() {
            for pipe_id in Self::PIPE_ID_MIN..Self::PIPE_ID_MAX {
                let socket_path = format!("{path}/{IPC_PIPE}{pipe_id}");
                if let Ok(socket) = UnixStream::connect(socket_path).await {
                    return Ok(socket);
                }
            }
        }
        Err(Error::Connection)
    }
}

/// Errors that may happen when using or making an IPC connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    /// Connection failed to initilaize
    ///
    /// This could be due to the Discord client not being opened in which the `discord-ipc-*` file
    /// is not created
    #[error("Failed to connect to any IPC stream! Discord may not be opened...")]
    Connection,
}

/// Client ID type used for the IPC connection.
#[derive(Debug, Clone)]
pub struct ClientId(pub String);

impl Default for ClientId {
    fn default() -> Self {
        Self(DEFAULT_CLIENT_ID.to_string())
    }
}

impl From<String> for ClientId {
    fn from(value: String) -> Self {
        ClientId(value)
    }
}

/// The file socket that Discord defines for IPC communication. This can be found in this specific
/// line in this (very old) [Discord IPC Documentation].
///
/// [Discord IPC Documentation](https://github.com/discord/discord-rpc/blob/master/documentation/hard-mode.md)
const IPC_PIPE: &str = "discord-ipc-";

/// Default client ID (this is for my own application :^))
const DEFAULT_CLIENT_ID: &str = "1276759902551015485";

#[cfg(target_family = "unix")]
/// Unix directories to check for the IPC named pipe
const PATH_DIRS: [Option<&'static str>; 5] = [
    option_env!("XDG_RUNTIME_DIR"),
    option_env!("TMPDIR"),
    option_env!("TMP"),
    option_env!("TEMP"),
    option_env!("TMP_DEFAULT"),
];

// TODO: add Windows support
// #[cfg(target_family = "windows")]
// const PATH_DIRS: [Option<&'static str>; 1] = [Some("asd")];
