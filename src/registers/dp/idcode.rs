use super::{DPRegister, ReadRegister};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idcode(u32);

impl DPRegister for Idcode {
    const A: [bool; 2] = [false, false];
}

impl From<u32> for Idcode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Idcode {
    pub fn version(&self) -> u32 {
        (self.0 >> 28) & 0xf
    }

    pub fn partno(&self) -> u32 {
        (self.0 >> 12) & 0xffff
    }

    pub fn designer(&self) -> u32 {
        (self.0 >> 1) & 0x7ff
    }

    pub fn present(&self) -> bool {
        self.0 & 1 == 1
    }
}

impl core::fmt::Debug for Idcode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Idcode")
            .field("raw", &self.0)
            .field("version()", &self.version())
            .field("partno()", &self.partno())
            .field("designer()", &self.designer())
            .field("present()", &self.present())
            .finish()
    }
}

impl ReadRegister for Idcode {}
