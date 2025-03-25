use crate::make_register;

use super::{DPRegister, ReadRegister};

make_register!(RdBuff, { (data, 0, 32) });

impl DPRegister for RdBuff {
    const A: [bool; 2] = [true, true];
}

impl ReadRegister for RdBuff {}
