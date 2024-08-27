use std::io;

use bytes::Buf;
use tokio_util::codec::Decoder as TokioDecoder;

use crate::{codec::MAX_FRAME_SIZE, opcode::Opcode};

use super::{IntermediateData, OPCODE_SIZE, PAYLOAD_SIZE};

#[derive(Debug, Default, Clone)]
pub(crate) struct Decoder;

impl TokioDecoder for Decoder {
    type Item = IntermediateData;

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

        if payload_len > MAX_FRAME_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                super::Error::PayloadTooLarge(payload_len),
            ));
        }

        if src.len() < OPCODE_SIZE + PAYLOAD_SIZE + payload_len {
            src.reserve(OPCODE_SIZE + PAYLOAD_SIZE + payload_len);
            return Ok(None);
        }

        let payload =
            src[(OPCODE_SIZE + PAYLOAD_SIZE)..(OPCODE_SIZE + PAYLOAD_SIZE + payload_len)].to_vec();
        src.advance(OPCODE_SIZE + PAYLOAD_SIZE + payload_len);
        Ok(Some(IntermediateData::new(opcode, payload).map_err(
            |err| io::Error::new(io::ErrorKind::InvalidData, err),
        )?))
    }
}
