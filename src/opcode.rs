#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum Opcode {
    Handshake = 0,
    Frame = 1,
    Close = 2,
}

impl From<u32> for Opcode {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::Handshake,
            1 => Self::Frame,
            2 => Self::Close,
            _ => panic!("Invalid opcodes that have not been implemented yet"),
        }
    }
}

impl From<Opcode> for u32 {
    fn from(v: Opcode) -> Self {
        v as u32
    }
}

impl From<[u8; 4]> for Opcode {
    fn from(value: [u8; 4]) -> Self {
        Opcode::from(u32::from_le_bytes(value))
    }
}

impl From<Opcode> for [u8; 4] {
    fn from(value: Opcode) -> Self {
        u32::from(value).to_le_bytes()
    }
}
