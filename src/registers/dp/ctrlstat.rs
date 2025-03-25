use crate::make_register;

use super::{DPRegister, ReadRegister, WriteRegister};

make_register!(CtrlStat, {
    (csyspwrupack, 31, 1, bool),
    (csyspwrupreq, 30, 1, bool),
    (cdbgpwrupack, 29, 1, bool),
    (cdbgpwrupreq, 28, 1, bool),
    (cdbgrstack, 27, 1, bool),
    (cdbgrstreq, 26, 1, bool)
});

impl DPRegister for CtrlStat {
    const A: [bool; 2] = [true, false];
}

impl ReadRegister for CtrlStat {}

impl WriteRegister for CtrlStat {}
