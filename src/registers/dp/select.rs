use crate::make_register;

use super::{DPRegister, ReadRegister, WriteRegister};

make_register!(Select, {
    (ctrlsel, 0, 1),
    (apbanksel, 4, 4, u8),
    (apsel, 24, 8, u8)
});

impl DPRegister for Select {
    const A: [bool; 2] = [false, true];
}

impl ReadRegister for Select {}

impl WriteRegister for Select {}
