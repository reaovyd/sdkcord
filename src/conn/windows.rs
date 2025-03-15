use super::ConnectionError;
use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::windows::named_pipe::{ClientOptions, NamedPipeClient},
    time,
};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
// https://github.com/biomejs/biome/blob/21dd358be5dade01d672916a0ab76bc825ef85cb/crates/biome_cli/src/service/windows.rs#L115
// taking from this as well since i can't figure this out haha

const DISCORD_WINDOWS_DIR: &str = r"\\?\pipe\discord-ipc-";
const IPC_CHANNELS: usize = 10;
const RETRY_ATTEMPTS: usize = 5;

pub(crate) async fn connect_windows() -> Result<(ClientReadHalf, ClientWriteHalf), ConnectionError>
{
    let client = Arc::new(get_client_connection().await?);
    let read_client = ClientReadHalf {
        inner: client.clone(),
    };
    let write_client = ClientWriteHalf { inner: client };
    Ok((read_client, write_client))
}

pub(crate) struct ClientWriteHalf {
    inner: Arc<NamedPipeClient>,
}

pub(crate) struct ClientReadHalf {
    inner: Arc<NamedPipeClient>,
}

impl AsyncRead for ClientReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            match self.inner.poll_read_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_read(buf.initialize_unfilled()) {
                    Ok(count) => {
                        buf.advance(count);
                        return Poll::Ready(Ok(()));
                    }

                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            };
        }
    }
}

impl AsyncWrite for ClientWriteHalf {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            match self.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_write(buf) {
                    Ok(count) => return Poll::Ready(Ok(count)),
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}

async fn get_client_connection() -> Result<NamedPipeClient, ConnectionError> {
    for channel in 0..IPC_CHANNELS {
        for _ in 0..RETRY_ATTEMPTS {
            match ClientOptions::new().open(format!("{DISCORD_WINDOWS_DIR}{channel}")) {
                Ok(client) => {
                    return Ok(client);
                }
                Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => {
                    time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
    Err(ConnectionError::ConnectionFailed)
}
