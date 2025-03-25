use super::{DPRegister, ReadRegister, WriteRegister};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CtrlStat(u32);

impl DPRegister for CtrlStat {
    const A: [bool; 2] = [true, false];
}

impl From<u32> for CtrlStat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ReadRegister for CtrlStat {}

impl From<CtrlStat> for u32 {
    fn from(value: CtrlStat) -> Self {
        value.0
    }
}

impl WriteRegister for CtrlStat {}

impl CtrlStat {
    pub fn csyspwrupack(&self) -> bool {
        (self.0 >> 31) & 1 == 1
    }

    pub fn csyspwrupreq(&self) -> bool {
        (self.0 >> 30) & 1 == 1
    }

    pub fn set_csyspwrupreq(self, val: bool) -> Self {
        Self((self.0 & !(1 << 30)) | ((val as u32) << 30))
    }

    pub fn cdbgpwrupack(&self) -> bool {
        (self.0 >> 29) & 1 == 1
    }

    pub fn cdbgpwrupreq(&self) -> bool {
        (self.0 >> 28) & 1 == 1
    }

    pub fn set_cdbgpwrupreq(self, val: bool) -> Self {
        Self((self.0 & !(1 << 28)) | ((val as u32) << 28))
    }

    pub fn cdbgrstack(&self) -> bool {
        (self.0 >> 27) & 1 == 1
    }

    pub fn cdbgrstreq(&self) -> bool {
        (self.0 >> 26) & 1 == 1
    }

    pub fn set_cdbgrstreq(self, val: bool) -> Self {
        Self((self.0 & !(1 << 26)) | ((val as u32) << 26))
    }
}

impl core::fmt::Debug for CtrlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CtrlStat")
            .field("raw", &self.0)
            .field("csyspwrupack()", &self.csyspwrupack())
            .field("cdbgpwrupack()", &self.cdbgpwrupack())
            .field("cdbgrstack()", &self.cdbgrstack())
            .finish()
    }
}
