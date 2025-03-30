use crate::{
    make_register,
    registers::ap::{APRegister, ReadRegister, WriteRegister},
};

make_register!(Tar, { (address, 0, 32) });

impl APRegister for Tar {
    const ADDRESS: u8 = 0x04;
}

impl ReadRegister for Tar {}
impl WriteRegister for Tar {}
