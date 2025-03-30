use crate::{
    make_register,
    registers::ap::{APRegister, ReadRegister, WriteRegister},
};

make_register!(Drw, { (data, 0, 32) });

impl APRegister for Drw {
    const ADDRESS: u8 = 0x0c;
}

impl ReadRegister for Drw {}
impl WriteRegister for Drw {}
