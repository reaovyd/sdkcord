use std::fmt::Display;

use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[repr(u32)]
pub enum Opcode {
    Handshake = 0,
    Frame = 1,
    Close = 2,
    Hello = 3,
}

impl TryFrom<u32> for Opcode {
    type Error = Error;

    fn try_from(opcode: u32) -> Result<Self, Self::Error> {
        match opcode {
            0 => Ok(Self::Handshake),
            1 => Ok(Self::Frame),
            2 => Ok(Self::Close),
            3 => Ok(Self::Hello),
            fail => Err(Error::InvalidOpcode(fail)),
        }
    }
}

impl From<Opcode> for u32 {
    fn from(opcode: Opcode) -> Self {
        opcode as u32
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum Error {
    #[error("Opcode {0} is not currently supported by the application.")]
    InvalidOpcode(u32),
}
