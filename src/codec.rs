use thiserror::Error;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::opcode::Opcode;

pub(crate) mod decoder;
pub(crate) mod encoder;

const MAX_FRAME_SIZE: usize = 1_000_000_000;
const OPCODE_SIZE: usize = std::mem::size_of::<Opcode>();
const PAYLOAD_SIZE: usize = std::mem::size_of::<u32>();

pub(crate) type IntermediateDataReceiver = Receiver<IntermediateData>;
pub(crate) type IntermediateDataSender = Sender<IntermediateData>;

#[derive(Debug, Clone, Hash)]
pub(crate) struct IntermediateData {
    opcode: Opcode,
    payload: Vec<u8>,
}

impl IntermediateData {
    #[inline(always)]
    pub(crate) fn new(opcode: Opcode, payload: Vec<u8>) -> Result<Self, Error> {
        if payload.len() > MAX_FRAME_SIZE {
            return Err(Error::PayloadTooLarge(payload.len()));
        }
        Ok(Self { opcode, payload })
    }

    #[inline(always)]
    pub(crate) const fn opcode(&self) -> Opcode {
        self.opcode
    }

    #[inline(always)]
    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Error, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub(crate) enum Error {
    #[error("Payload too large! Received {0} bytes")]
    PayloadTooLarge(usize),
}
