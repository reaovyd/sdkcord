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

impl Opcode {
    pub const fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub const fn into_u32(self) -> u32 {
        self as u32
    }
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::Opcode;

    #[test]
    fn test_opcode_display() {
        [
            (Opcode::Handshake, "Handshake"),
            (Opcode::Frame, "Frame"),
            (Opcode::Close, "Close"),
            (Opcode::Hello, "Hello"),
        ]
        .into_iter()
        .for_each(|(opcode, expected)| {
            assert_eq!(format!("{}", opcode), expected.to_string());
        });
    }

    #[test]
    fn test_convert_opcode_to_u32() {
        [(Opcode::Handshake, 0), (Opcode::Frame, 1), (Opcode::Close, 2), (Opcode::Hello, 3)]
            .into_iter()
            .for_each(|(opcode, expected)| {
                assert_eq!(opcode as u32, expected);
            });
    }

    #[test]
    fn test_convert_u32_to_opcode_should_succeed() {
        [(0, Opcode::Handshake), (1, Opcode::Frame), (2, Opcode::Close), (3, Opcode::Hello)]
            .into_iter()
            .for_each(|(actual, expected_opcode)| {
                assert_eq!(Opcode::try_from(actual).unwrap(), expected_opcode);
            });
    }

    #[test]
    #[should_panic(expected = "InvalidOpcode")]
    fn test_convert_u32_to_opcode_should_fail() {
        let num = 34;
        Opcode::try_from(num).unwrap();
    }
}
