use std::{error::Error, io::ErrorKind};

use futures::StreamExt;
use tokio::net::unix::OwnedReadHalf;
use tokio_util::codec::FramedRead;

use crate::codec::{decoder::Decoder, Error as CodecError, IntermediateDataSender};

type IPCReader = OwnedReadHalf;

#[derive(Debug)]
pub(crate) struct Reader {
    inner: FramedRead<IPCReader, Decoder>,
    de_tx: IntermediateDataSender,
}

impl Reader {
    #[inline(always)]
    pub(crate) fn new(reader: IPCReader, decoder: Decoder, de_tx: IntermediateDataSender) -> Self {
        let inner = FramedRead::new(reader, decoder);
        Self { inner, de_tx }
    }

    pub(crate) async fn start_loop(mut self) {
        while let Some(resp) = self.inner.next().await {
            match resp {
                Ok(intrm_data) => {
                    let sender = self.de_tx.clone();
                    tokio::spawn(async move {
                        // TODO: panic or log? think i'll slap panic for now?
                        sender.send(intrm_data).await.expect("reader half is dead");
                    });
                }
                Err(err) => {
                    if let ErrorKind::InvalidData = err.kind() {
                        continue;
                    } else {
                        // TODO: panic or break? will just break for now? but definitely log here
                        break;
                    }
                }
            };
        }
    }
}
