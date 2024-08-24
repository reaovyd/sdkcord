//! Module that provides functionality to handle the IPC connection.

use thiserror::Error;
use tokio::net::UnixStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Failed to connect to any IPC stream! Discord may not be opened...")]
    Connection,
}

#[derive(Debug)]
pub struct Connection {
    inner: UnixStream,
}

impl Connection {
    const MIN_PIPE_ID: usize = 0;
    const MAX_PIPE_ID: usize = 9;

    pub async fn connect() -> Result<Self, Error> {
        let inner = Self::get_connected_stream().await?;
        Ok(Self { inner })
    }

    async fn get_connected_stream() -> Result<UnixStream, Error> {
        for path in PATH_DIRS.into_iter().flatten() {
            for pipe_id in Self::MIN_PIPE_ID..=Self::MAX_PIPE_ID {
                let socket_path = format!("{path}/{IPC_PIPE}{pipe_id}");
                if let Ok(socket) = UnixStream::connect(socket_path).await {
                    return Ok(socket);
                }
            }
        }
        Err(Error::Connection)
    }
}

/// Client ID type used for the IPC connection.
#[derive(Debug, Clone)]
pub struct ClientId(pub String);

impl Default for ClientId {
    fn default() -> Self {
        Self(DEFAULT_CLIENT_ID.to_string())
    }
}

/// The file socket that Discord defines for IPC communication. This can be found in this specific
/// line in this (very old) [Discord IPC Documentation].
///
/// [Discord IPC Documentation](https://github.com/discord/discord-rpc/blob/master/documentation/hard-mode.md)
const IPC_PIPE: &str = "discord-ipc-";

/// Default client ID (this is my own application :^))
const DEFAULT_CLIENT_ID: &str = "1276759902551015485";

#[cfg(target_family = "unix")]
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
