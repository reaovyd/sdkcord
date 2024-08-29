use std::io;

use bytes::BufMut;
use tokio_util::codec::Encoder as TokioEncoder;
use tracing::{instrument, trace};

use super::IntermediateData;

#[derive(Debug, Default, Clone)]
pub(crate) struct Encoder;

impl TokioEncoder<IntermediateData> for Encoder {
    type Error = io::Error;

    #[instrument(name = "codec::Encoder::encode", skip(self, dst), level = "trace")]
    fn encode(
        &mut self,
        item: IntermediateData,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let opcode = item.opcode();
        let payload = item.payload();
        let payload_len = payload.len();

        trace!(
            "Storing opcode: {}, payload_len: {}, payload {:?} inside buffer",
            opcode,
            payload_len,
            payload
        );
        dst.put_u32_le(opcode as u32);
        dst.put_u32_le(payload_len as u32);
        dst.put(payload);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use pretty_assertions::assert_eq;
    use tokio_util::codec::Encoder as _;

    use crate::{
        codec::{IntermediateData, OPCODE_SIZE, PAYLOAD_SIZE},
        opcode::Opcode,
    };

    use super::Encoder;

    #[test]
    fn test_encoder_basic() {
        let mut encoder = Encoder {};
        let mut bytes_mut = BytesMut::new();
        let intermediate_data =
            IntermediateData::new(Opcode::Hello, vec![b'a', b'c', b'd']).unwrap();
        encoder.encode(intermediate_data, &mut bytes_mut).unwrap();
        // 4 bytes for opcode, 4 bytes for payload size, 3 bytes for payload
        assert_eq!(bytes_mut.len(), 11);
        // must be little endian
        assert_eq!(bytes_mut[..OPCODE_SIZE], [3, 0, 0, 0]);
        assert_eq!(
            bytes_mut[OPCODE_SIZE..(OPCODE_SIZE + PAYLOAD_SIZE)],
            [3, 0, 0, 0]
        );
    }
}
