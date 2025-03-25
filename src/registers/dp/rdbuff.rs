use super::{DPRegister, ReadRegister};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RdBuff(pub u32);

impl DPRegister for RdBuff {
    const A: [bool; 2] = [true, true];
}

impl From<u32> for RdBuff {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ReadRegister for RdBuff {}
