pub trait DPRegister {
    const A: [bool; 2];
}

pub trait ReadRegister: DPRegister + From<u32> + core::fmt::Debug {}

pub trait WriteRegister: DPRegister + Into<u32> + core::fmt::Debug {}

pub mod idcode;
pub use idcode::Idcode;

pub mod ctrlstat;
pub use ctrlstat::CtrlStat;

pub mod select;
pub use select::Select;

pub mod rdbuff;
pub use rdbuff::RdBuff;
