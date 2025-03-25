use crate::make_register;

use super::{DPRegister, ReadRegister};

make_register!(Idcode, {
    (version, 28, 4),
    (partno, 12, 16),
    (designer, 1, 11),
    (present, 0, 1, bool)
});

impl DPRegister for Idcode {
    const A: [bool; 2] = [false, false];
}

impl ReadRegister for Idcode {}
