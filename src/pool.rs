use std::thread;

use async_channel::Sender;
use bon::builder;
use thiserror::Error;
use tokio::sync::oneshot::{Sender as OneshotSender, error::RecvError};
use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub(crate) struct Client<M, R>(Sender<(M, OneshotSender<R>)>);

impl<M, R> Client<M, R>
where
    M: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub(crate) async fn deserialize(&self, data: M) -> Result<R, SerdePoolError> {
        self.send(data).await
    }
    pub(crate) async fn serialize(&self, data: M) -> Result<R, SerdePoolError> {
        self.send(data).await
    }
    #[inline(always)]
    async fn send(&self, data: M) -> Result<R, SerdePoolError> {
        let (sndr, recv) = tokio::sync::oneshot::channel();
        self.0
            .send((data, sndr))
            .await
            .map_err(|_| SerdePoolError::PoolSend)?;
        recv.await.map_err(SerdePoolError::OneshotRecv)
    }
}

#[builder]
#[instrument(skip(num_threads, op))]
pub(crate) fn spawn_pool<F, M, R>(num_threads: u8, op: F, cap: usize) -> Client<M, R>
where
    F: Fn(&M) -> R + Send + Clone + 'static,
    M: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    let (sndr, recv) = async_channel::bounded::<(M, OneshotSender<R>)>(cap);
    thread::spawn(move || {
        let handlers = (0..num_threads).map(|_| {
            let op = op.clone();
            let recv = recv.clone();
            thread::spawn(move || loop {
                if let Ok((data, sender)) = recv.recv_blocking() {
                    if sender.send(op(&data)).is_err() {
                        error!("sender failed to send job response data! the receiving task may have likely died before it received the value.");
                    }
                } else {
                    error!("channel is closed. closing receiver end and exiting");
                    recv.close();
                    break;
                }
            })
        }).collect::<Vec<_>>();
        for thread in handlers {
            thread.join().unwrap();
        }
    });
    Client(sndr)
}

#[derive(Debug, Clone, Error)]
pub(crate) enum SerdePoolError {
    #[error("the pool could not receive the response as pool channel has been closed")]
    PoolSend,
    #[error("the oneshot channel sender has been killed and channel is closed without messages")]
    OneshotRecv(#[from] RecvError),
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::spawn_pool;

    #[inline(always)]
    const fn op(num: &u32) -> u32 {
        *num + 1
    }

    #[tokio::test]
    async fn test_pool_spawn_op() {
        let sender = spawn_pool().num_threads(8).op(op).cap(8).call();
        let handlers = (0..4)
            .map(|_| {
                let tx = sender.clone();
                tokio::spawn(async move { tx.send(3).await.unwrap() })
            })
            .collect::<Vec<_>>();
        for handler in handlers {
            assert_eq!(4, handler.await.unwrap());
        }
    }
}
