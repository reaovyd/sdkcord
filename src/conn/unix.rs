//! A module that provides methods to connect to the IPC socket on Unix systems
use std::env;

use tokio::net::{unix::OwnedReadHalf, unix::OwnedWriteHalf};

use super::ConnectionError;

// shamefully taking this from https://github.com/vionya/discord-rich-presence/blob/main/src/ipc_unix.rs
// since i can't figure this out haha

/// Exhaustive list of folder directories for where the IPC socket may lie on a Unix system
const DISCORD_UNIX_DIRS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];
/// Exhaustive list of paths for where the IPC socket may lie on a Unix system
const DISCORD_UNIX_PATHS: [&str; 4] = [
    "",
    "app/com.discordapp.Discord/",
    "snap.discord-canary/",
    "snap.discord/",
];
/// Prefix for the IPC socket and the suffix is a number from 0 to 9
const DISCORD_IPC_PATH: &str = "discord-ipc-";

/// Connects to the Discord IPC socket on a Unix system
///
/// This does an exhaustive search for the IPC socket in the directories specified in
/// [DISCORD_UNIX_DIRS] and [DISCORD_UNIX_PATHS]
///
/// # Errors
/// A [ConnectionError] will be returned if the connection fails
pub(crate) async fn connect_unix() -> Result<(OwnedReadHalf, OwnedWriteHalf), ConnectionError> {
    for dir in DISCORD_UNIX_DIRS {
        if let Ok(dir) = env::var(dir) {
            for path in DISCORD_UNIX_PATHS {
                for channel_num in 0..10 {
                    let path = format!("{}/{}{DISCORD_IPC_PATH}{}", dir, path, channel_num);
                    if let Ok(stream) = tokio::net::UnixStream::connect(path).await {
                        return Ok(stream.into_split());
                    }
                }
            }
        }
    }
    Err(ConnectionError::ConnectionFailed)
}
