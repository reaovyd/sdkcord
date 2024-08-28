use futures::SinkExt;
use tokio::net::unix::OwnedWriteHalf;
use tokio_util::codec::FramedWrite;

use crate::codec::{encoder::Encoder, IntermediateDataReceiver};

type IPCWriter = OwnedWriteHalf;

#[derive(Debug)]
pub(crate) struct Writer {
    inner: FramedWrite<IPCWriter, Encoder>,
    ser_rx: IntermediateDataReceiver,
}

impl Writer {
    #[inline(always)]
    pub(crate) fn new(
        writer: IPCWriter,
        encoder: Encoder,
        ser_rx: IntermediateDataReceiver,
    ) -> Self {
        let inner = FramedWrite::new(writer, encoder);
        Self { inner, ser_rx }
    }

    pub(crate) async fn start_loop(mut self) {
        while let Some(intrm_data) = self.ser_rx.recv().await {
            if let Err(err) = self.inner.send(intrm_data).await {
                // TODO: panic or break? but definitely log
                break;
            } else {
                // TODO: log here
            }
        }
    }
}
