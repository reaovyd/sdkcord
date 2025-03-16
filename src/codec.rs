use std::io;

use bytes::{Buf, BufMut, Bytes};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

use crate::payload::common::opcode::Opcode;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Frame {
    pub(crate) opcode: Opcode,
    pub(crate) len: u32,
    pub(crate) payload: Bytes,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FrameCodec;

const MAX_FRAME_LENGTH: usize = 1_000_000_000;
const OPCODE_SIZE: usize = std::mem::size_of::<Opcode>();
const PAYLOAD_SIZE: usize = std::mem::size_of::<u32>();

impl Encoder<Frame> for FrameCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Frame, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        if item.len + (OPCODE_SIZE + PAYLOAD_SIZE) as u32 > MAX_FRAME_LENGTH as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Frame of length is too large: {}", item.len),
            ));
        }
        dst.put_u32_le(item.opcode as u32);
        dst.put_u32_le(item.len);
        dst.put(item.payload);
        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;

    type Error = io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < OPCODE_SIZE + PAYLOAD_SIZE {
            return Ok(None);
        }
        let mut opcode_bytes = [0u8; OPCODE_SIZE];
        opcode_bytes.copy_from_slice(&src[..OPCODE_SIZE]);
        let opcode = Opcode::try_from(u32::from_le_bytes(opcode_bytes))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        let mut payload_len_bytes = [0u8; PAYLOAD_SIZE];
        payload_len_bytes.copy_from_slice(&src[OPCODE_SIZE..OPCODE_SIZE + PAYLOAD_SIZE]);
        let payload_len = u32::from_le_bytes(payload_len_bytes) as usize;

        if payload_len > MAX_FRAME_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                Error::PayloadTooLarge(payload_len),
            ));
        }

        if src.len() < OPCODE_SIZE + PAYLOAD_SIZE + payload_len {
            src.reserve(OPCODE_SIZE + PAYLOAD_SIZE + payload_len);
            return Ok(None);
        }

        let payload =
            src[(OPCODE_SIZE + PAYLOAD_SIZE)..(OPCODE_SIZE + PAYLOAD_SIZE + payload_len)].to_vec();
        src.advance(OPCODE_SIZE + PAYLOAD_SIZE + payload_len);
        Ok(Some(Frame {
            opcode,
            len: payload_len as u32,
            payload: Bytes::from(payload),
        }))
    }
}

#[derive(Debug, Error, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Error {
    #[error("Payload too large! Received {0} bytes")]
    PayloadTooLarge(usize),
}
