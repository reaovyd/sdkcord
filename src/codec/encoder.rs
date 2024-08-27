use std::io;

use bytes::BufMut;
use thiserror::Error;
use tokio_util::codec::Encoder as TokioEncoder;

use super::IntermediateData;

#[derive(Debug, Default, Clone)]
pub(crate) struct Encoder;

impl TokioEncoder<IntermediateData> for Encoder {
    type Error = Error;

    fn encode(
        &mut self,
        item: IntermediateData,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let opcode = item.opcode() as u32;
        let payload = item.payload();
        let payload_len = payload.len() as u32;

        dst.put_u32_le(opcode);
        dst.put_u32_le(payload_len);
        dst.put(payload);

        Ok(())
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub(crate) enum Error {
    #[error("Encoding error: {0}")]
    EncodeError(#[from] io::Error),
}
