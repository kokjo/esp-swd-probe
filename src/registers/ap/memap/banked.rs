use crate::{
    make_register,
    registers::ap::{APRegister, ReadRegister, WriteRegister},
};

make_register!(BD0, { (data, 0, 32) });

impl APRegister for BD0 {
    const ADDRESS: u8 = 0x10;
}

impl ReadRegister for BD0 {}
impl WriteRegister for BD0 {}

make_register!(BD1, { (data, 0, 32) });

impl APRegister for BD1 {
    const ADDRESS: u8 = 0x14;
}

impl ReadRegister for BD1 {}
impl WriteRegister for BD1 {}

make_register!(BD2, { (data, 0, 32) });

impl APRegister for BD2 {
    const ADDRESS: u8 = 0x18;
}

impl ReadRegister for BD2 {}
impl WriteRegister for BD2 {}

make_register!(BD3, { (data, 0, 32) });

impl APRegister for BD3 {
    const ADDRESS: u8 = 0x18;
}

impl ReadRegister for BD3 {}
impl WriteRegister for BD3 {}
