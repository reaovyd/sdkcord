use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use strum_macros::EnumString;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
)]
#[repr(u32)]
pub enum Opcode {
    Handshake = 0,
    Frame = 1,
    Close = 2,
    Hello = 3,
}
