use crate::codec::IntermediateDataReceiver;

use super::PayloadResponseSender;

#[derive(Debug)]
pub(crate) struct Deserializer {
    // placeholder unit type for now
    bridge_tx: PayloadResponseSender,
    reader_rx: IntermediateDataReceiver,
}

impl Deserializer {
    pub(crate) const fn new(
        bridge_tx: PayloadResponseSender,
        reader_rx: IntermediateDataReceiver,
    ) -> Self {
        Self { bridge_tx, reader_rx }
    }

    pub(crate) fn start_loop(mut self) {
        while let Some(data) = self.reader_rx.blocking_recv() {}
    }
}
