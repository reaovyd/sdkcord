use std::io::ErrorKind;

use futures::StreamExt;
use tokio::net::unix::OwnedReadHalf;
use tokio_util::codec::FramedRead;
use tracing::{error, instrument, trace};

use crate::codec::{decoder::Decoder, IntermediateData, IntermediateDataSender};

type IPCSocket = OwnedReadHalf;
type IPCReader = FramedRead<IPCSocket, Decoder>;

#[derive(Debug)]
pub(crate) struct Reader {
    inner: IPCReader,
    de_tx: IntermediateDataSender,
}

impl Reader {
    #[instrument(
        "ipc::reader::Reader::new",
        skip(reader, decoder, de_tx),
        level = "trace"
    )]
    pub(crate) fn new(reader: IPCSocket, decoder: Decoder, de_tx: IntermediateDataSender) -> Self {
        let inner = FramedRead::new(reader, decoder);
        Self { inner, de_tx }
    }

    #[instrument("ipc::reader::Reader::start_loop", skip(self), level = "trace")]
    pub(crate) async fn start_loop(mut self) {
        while let Some(resp) = self.inner.next().await {
            trace!("Received decoded data bytes from IPC");
            match resp {
                Ok(intrm_data) => {
                    trace!("Intermediate data received was successful. Spawning task to send...");
                    let sender = self.de_tx.clone();
                    tokio::spawn(send(sender, intrm_data));
                }
                Err(err) => {
                    if let ErrorKind::InvalidData = err.kind() {
                        error!("Error had invalid data (continuing...): {}", err);
                        continue;
                    } else {
                        error!("Error (quitting...): {}", err);
                        break;
                    }
                }
            };
        }
    }
}

#[instrument("ipc::reader::Reader::send", skip(sender, intrm_data), level = "trace")]
async fn send(sender: IntermediateDataSender, intrm_data: IntermediateData) {
    trace!("Sending intermediate data over to deserializer...");
    sender
        .send(intrm_data)
        .await
        .expect("reader half is dead...")
}
