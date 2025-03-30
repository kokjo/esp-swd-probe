use crate::{
    make_register,
    registers::ap::{APRegister, ReadRegister, WriteRegister},
};

make_register!(CSW, {
    (dbgswenable, 31, 1, bool),
    (prot, 24, 7, u8),
    (spiden, 23, 1, bool),
    (mte, 15, 1, bool),
    (access_type, 12, 3),
    (mode, 8, 4),
    (trinprog, 7, 1, bool),
    (deviceen, 6, 1, bool),
    (addrinc, 4, 2),
    (size, 0, 3)
});

impl APRegister for CSW {
    const ADDRESS: u8 = 0x00;
}

impl ReadRegister for CSW {}
impl WriteRegister for CSW {}
