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

impl From<u8> for APType {
    fn from(value: u8) -> Self {
        match value {
            0 => APType::Jtag,
            1 => APType::AHB,
            2 => APType::APB,
            x => APType::Unknown(x),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum APClass {
    NoClass,
    ComAP,
    MemAp,
    Unknown(u8),
}

impl From<u8> for APClass {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => APClass::NoClass,
            0b0001 => APClass::ComAP,
            0b1000 => APClass::MemAp,
            x => APClass::Unknown(x),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idr(pub u32);

impl APRegister for Idr {
    const ADDRESS: u8 = 0xfc;
}

impl From<u32> for Idr {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ReadRegister for Idr {}

impl Idr {
    pub fn ap_type(&self) -> APType {
        ((self.0 & 0xf) as u8).into()
    }

    pub fn variant(&self) -> u8 {
        ((self.0 >> 4) & 0xf) as u8
    }

    pub fn class(&self) -> APClass {
        (((self.0 >> 13) & 0xf) as u8).into()
    }

    pub fn revision(&self) -> u8 {
        ((self.0 >> 28) & 0xf) as u8
    }

    pub fn designer(&self) -> u32 {
        (self.0 >> 17) & 0x7ff
    }

    pub fn is_mem_ap(&self) -> bool {
        self.class() == APClass::MemAp
    }

    pub fn is_jtag_connection(&self) -> bool {
        self.ap_type() == APType::Jtag && self.class() == APClass::NoClass && self.variant() != 0
    }
}

impl core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Idr")
            .field("raw", &self.0)
            .field("ap_type()", &self.ap_type())
            .field("class()", &self.class())
            .field("variant()", &self.variant())
            .field("revision()", &self.revision())
            .field("designer()", &self.designer())
            .finish()
    }
}

pub mod memap;
