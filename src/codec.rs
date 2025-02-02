use tokio_util::codec::{Decoder, Encoder};

use crate::payload::common::opcode::Opcode;

#[derive(Debug)]
pub(crate) struct FrameCodec {}

const MAX_FRAME_LENGTH: u32 = 1_000_000_000;
const OPCODE_SIZE: u32 = std::mem::size_of::<Opcode>() as u32;
const PAYLOAD_SIZE: u32 = std::mem::size_of::<u32>() as u32;

// impl Encoder<String> for FrameCodec {
//     type Error;
//
//     fn encode(&mut self, item: String, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
//         todo!()
//     }
// }
//
// impl Decoder<String> for FrameCodec {
//     type Item;
//
//     type Error;
//
//     fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
//         todo!()
//     }
// }
