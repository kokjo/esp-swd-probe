use super::{DPRegister, ReadRegister, WriteRegister};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Select(u32);

impl DPRegister for Select {
    const A: [bool; 2] = [false, true];
}

impl From<u32> for Select {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ReadRegister for Select {}

impl From<Select> for u32 {
    fn from(value: Select) -> Self {
        value.0
    }
}

impl WriteRegister for Select {}

impl Select {
    pub fn ctrlsel(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn set_ctrlsel(self, ctrlsel: bool) -> Self {
        Self((self.0 & !(1 << 0)) | (ctrlsel as u32))
    }

    pub fn apbanksel(&self) -> u8 {
        ((self.0 >> 4) & 0x0f) as u8
    }

    pub fn set_apbanksel(self, bank: u8) -> Self {
        Self((self.0 & !(0xf << 4)) | (((bank as u32) & 0xf) << 4))
    }

    pub fn apsel(&self) -> u8 {
        ((self.0 >> 24) & 0xff) as u8
    }

    pub fn set_apsel(self, ap: u8) -> Self {
        Self((self.0 & !(0xff << 24)) | ((ap as u32) << 24))
    }
}

impl core::fmt::Debug for Select {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Select")
            .field("raw", &self.0)
            .field("apsel", &self.apsel())
            .field("apbanksel", &self.apbanksel())
            .field("ctrlsel", &self.ctrlsel())
            .finish()
    }
}
