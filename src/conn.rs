// shamefully taking this from https://github.com/vionya/discord-rich-presence/blob/main/src/ipc_unix.rs
// since i can't figure this out haha

use thiserror::Error;

#[cfg(unix)]
mod unix {
    use std::env;

    use tokio::net::{unix::OwnedReadHalf, unix::OwnedWriteHalf};

    use super::ConnectionError;

    const DISCORD_UNIX_DIRS: [&str; 4] = ["XGD_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];
    const DISCORD_UNIX_PATHS: [&str; 4] = [
        "",
        "app/com.discordapp.Discord/",
        "snap.discord-canary/",
        "snap.discord/",
    ];

    async fn connect_unix() -> Result<(OwnedReadHalf, OwnedWriteHalf), ConnectionError> {
        for dir in DISCORD_UNIX_DIRS {
            if let Ok(dir) = env::var(dir) {
                for path in DISCORD_UNIX_PATHS {
                    for channel_num in 0..10 {
                        let path = format!("{}/{}{}", dir, path, channel_num);
                        if let Ok(stream) = tokio::net::UnixStream::connect(path).await {
                            return Ok(stream.into_split());
                        }
                    }
                }
            }
        }
        Err(ConnectionError::ConnectionFailed)
    }
}

#[cfg(windows)]
mod windows {
    const DISCORD_WINDOWS_DIRS: [&str; 1] = [r"\\?\pipe\discord-ipc-"];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ConnectionError {
    #[error(
        "failed to connect after trying all possible paths! if you think the IPC path on your system lives elsewhere, please open an issue on the repository!"
    )]
    ConnectionFailed,
}
