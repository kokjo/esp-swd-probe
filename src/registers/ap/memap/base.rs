use crate::registers::ap::{APRegister, ReadRegister};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base(u32);

impl APRegister for Base {
    const ADDRESS: u8 = 0xf8;
}

impl From<u32> for Base {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ReadRegister for Base {}

impl Base {
    pub fn address(&self) -> u32 {
        self.0 & 0xfffff000
    }

    pub fn present(&self) -> bool {
        (self.0 & 1) == 1
    }
}

impl core::fmt::Debug for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Base")
            .field("raw", &self.0)
            .field("present()", &self.present())
            .field("address()", &self.address())
            .finish()
    }
}
