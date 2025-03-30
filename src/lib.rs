#![no_std]

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub(crate) use mk_static;

pub mod memap;
pub mod registers;
pub mod swd;

pub mod net;
pub mod wifi;

pub(crate) use registers::make_register;
