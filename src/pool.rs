use std::thread;

use async_channel::Receiver;
use tokio::sync::oneshot::Sender as OneshotSender;
use tracing::{error, instrument};

#[derive(Debug)]
pub(crate) struct Pool<F, M, R> {
    num_threads: u8,
    op: F,
    recv: Receiver<(M, OneshotSender<R>)>,
}

impl<F, M, R> Pool<F, M, R>
where
    F: Fn(&M) -> R + Send + Clone + 'static,
    M: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub(crate) fn new(num_threads: u8, op: F, recv: Receiver<(M, OneshotSender<R>)>) -> Self {
        Self { num_threads, op, recv }
    }

    #[instrument(skip(self))]
    pub(crate) fn run(self) {
        let handlers = (0..self.num_threads).map(|_| {
            let recv = self.recv.clone();
            let op = self.op.clone();
            thread::spawn(move || loop {
                if let Ok((data, sender)) = recv.recv_blocking() {
                    if sender.send(op(&data)).is_err() {
                        error!("sender failed to send job response data! the receiving task may have likely died before it received the value.");
                    }
                } else {
                    // TODO: get some kind of tracing in here?
                    error!("channel is closed. closing receiver end and exiting");
                    recv.close();
                    break;
                }
            })
        }).collect::<Vec<_>>();
        for thread in handlers {
            thread.join().unwrap()
        }
        self.recv.close();
    }
}
