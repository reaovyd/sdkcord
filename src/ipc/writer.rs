use std::io;

use futures::SinkExt;
use tokio::net::unix::OwnedWriteHalf;
use tokio_util::codec::FramedWrite;
use tracing::{error, info, instrument, trace};

use crate::codec::{encoder::Encoder, IntermediateData, IntermediateDataReceiver};

type IPCSocket = OwnedWriteHalf;
type IPCWriter = FramedWrite<IPCSocket, Encoder>;

#[derive(Debug)]
pub(crate) struct Writer {
    inner: IPCWriter,
    ser_rx: IntermediateDataReceiver,
}

impl Writer {
    #[instrument(
        "ipc::writer::Writer::new",
        skip(writer, encoder, ser_rx),
        level = "trace"
    )]
    pub(crate) fn new(
        writer: IPCSocket,
        encoder: Encoder,
        ser_rx: IntermediateDataReceiver,
    ) -> Self {
        let inner = FramedWrite::new(writer, encoder);
        Self { inner, ser_rx }
    }

    #[instrument("ipc::writer::Writer::start_loop", skip(self), level = "trace")]
    pub(crate) async fn start_loop(mut self) {
        while let Some(intrm_data) = self.ser_rx.recv().await {
            trace!("Received intermediate data from the serializer");
            if let Err(err) = send(&mut self.inner, intrm_data).await {
                error!("Error: quitting now because of {err}");
                break;
            } else {
                info!("Successfully sent intermediate data to IPC")
            }
        }
    }
}

#[instrument("ipc::writer::Writer::send", skip(sender, intrm_data), level = "trace")]
async fn send(sender: &mut IPCWriter, intrm_data: IntermediateData) -> Result<(), io::Error> {
    sender.send(intrm_data).await
}
