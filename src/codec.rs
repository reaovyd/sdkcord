use thiserror::Error;

use crate::opcode::Opcode;

pub(crate) mod decoder;
pub(crate) mod encoder;

const MAX_FRAME_SIZE: usize = 1_000_000_000;
const OPCODE_SIZE: usize = std::mem::size_of::<Opcode>();
const PAYLOAD_SIZE: usize = std::mem::size_of::<u32>();

#[derive(Debug, Clone, Hash)]
pub(crate) struct IntermediateData {
    opcode: Opcode,
    payload: Vec<u8>,
}

impl IntermediateData {
    #[inline(always)]
    pub(crate) fn new(opcode: Opcode, payload: Vec<u8>) -> Result<Self, Error> {
        if OPCODE_SIZE + PAYLOAD_SIZE + payload.len() > MAX_FRAME_SIZE {
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
#[non_exhaustive]
pub(crate) enum Error {
    #[error("Payload too large! Received {0} bytes")]
    PayloadTooLarge(usize),
}
