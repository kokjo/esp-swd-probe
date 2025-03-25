use crate::make_register;

pub trait APRegister {
    const ADDRESS: u8;
}

pub trait ReadRegister: APRegister + From<u32> + core::fmt::Debug {}
pub trait WriteRegister: APRegister + Into<u32> + core::fmt::Debug {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum APType {
    Jtag,
    AHB,
    APB,
    Unknown(u8),
}

impl From<u32> for APType {
    fn from(value: u32) -> Self {
        match value {
            0 => APType::Jtag,
            1 => APType::AHB,
            2 => APType::APB,
            x => APType::Unknown(x as u8),
        }
    }
}

impl From<APType> for u32 {
    fn from(value: APType) -> Self {
        match value {
            APType::Jtag => 0,
            APType::AHB => 1,
            APType::APB => 2,
            APType::Unknown(x) => x as u32,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum APClass {
    NoClass,
    ComAp,
    MemAp,
    Unknown(u8),
}

impl From<u32> for APClass {
    fn from(value: u32) -> Self {
        match value {
            0b0000 => APClass::NoClass,
            0b0001 => APClass::ComAp,
            0b1000 => APClass::MemAp,
            x => APClass::Unknown(x as u8),
        }
    }
}

impl From<APClass> for u32 {
    fn from(value: APClass) -> Self {
        match value {
            APClass::NoClass => 0b0000,
            APClass::ComAp => 0b0001,
            APClass::MemAp => 0b1000,
            APClass::Unknown(x) => x as u32,
        }
    }
}

make_register!(Idr, {
    (ap_type, 0, 4, APType),
    (variant, 4, 4),
    (class, 13, 4, APClass),
    (revision, 28, 4),
    (designer, 17, 11)
});

impl APRegister for Idr {
    const ADDRESS: u8 = 0xfc;
}

impl ReadRegister for Idr {}

impl Idr {
    pub fn is_mem_ap(&self) -> bool {
        self.class() == APClass::MemAp
    }

    pub fn is_jtag_connection(&self) -> bool {
        self.ap_type() == APType::Jtag && self.class() == APClass::NoClass && self.variant() != 0
    }
}


pub mod memap;
