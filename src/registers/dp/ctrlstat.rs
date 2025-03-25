use crate::make_register;

use super::{DPRegister, ReadRegister, WriteRegister};

make_register!(CtrlStat, {
    (csyspwrupack, 31, 1, bool),
    (csyspwrupreq, 30, 1, bool),
    (cdbgpwrupack, 29, 1, bool),
    (cdbgpwrupreq, 28, 1, bool),
    (cdbgrstack, 27, 1, bool),
    (cdbgrstreq, 26, 1, bool),
    (trncnt, 12, 12),
    (masklane, 8, 4),
    (wdataerr, 7, 1, bool),
    (readok, 6, 1, bool),
    (stickyerr, 5, 1, bool),
    (stickycmp, 4, 1, bool),
    (trnmode, 2, 2, bool),
    (stickyorun, 1, 1, bool),
    (orundetect, 0, 1, bool)
});

impl DPRegister for CtrlStat {
    const A: [bool; 2] = [true, false];
}

impl ReadRegister for CtrlStat {}

impl WriteRegister for CtrlStat {}
