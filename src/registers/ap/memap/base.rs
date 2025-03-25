use crate::{
    make_register,
    registers::ap::{APRegister, ReadRegister},
};

make_register!(Base, { (present, 0, 1, bool) });

impl APRegister for Base {
    const ADDRESS: u8 = 0xf8;
}

impl ReadRegister for Base {}

impl Base {
    pub fn address(&self) -> u32 {
        self.0 & 0xfffff000
    }
}
