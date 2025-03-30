use crate::{registers::ap::{memap::{Base, Drw, Tar}, ReadRegister, WriteRegister}, swd::{RequestError, Swd}};


pub struct MemAp<'swd, 'pins> {
    swd: &'swd mut Swd<'pins>,
    ap: u8,
} 

impl<'pins> Swd<'pins> {
    pub fn memap<'swd>(&'swd mut self, ap: u8) -> MemAp<'swd, 'pins> {
        MemAp {
            swd: self,
            ap: ap
        }
    }
}

impl MemAp<'_, '_> {
    pub async fn write_register<Reg: WriteRegister>(&mut self, reg: Reg) -> Result<(), RequestError> {
        self.swd.write_ap_register(self.ap, reg).await
    }

    pub async fn read_register<Reg: ReadRegister>(&mut self) -> Result<Reg, RequestError> {
        self.swd.read_ap_register(self.ap).await
    }

    pub async fn modify_register<Reg: ReadRegister + WriteRegister>(&mut self, f: impl FnOnce(Reg) -> Reg) -> Result<(), RequestError> {
        let old_reg = self.read_register().await?;
        let new_reg = f(old_reg);
        self.write_register(new_reg).await
    }

    pub async fn base(&mut self) -> Result<Base, RequestError> {
        self.read_register().await
    }

    pub async fn read_32(&mut self, address: u32) -> Result<u32, RequestError> {
        self.write_register(Tar::default().set_address(address)).await?;
        Ok(self.read_register::<Drw>().await?.data())
    }

    pub async fn write_32(&mut self, address: u32, value: u32) -> Result<(), RequestError> {
        self.write_register(Tar::default().set_address(address)).await?;
        self.write_register(Drw::default().set_data(value)).await?;
        Ok(())
    }
}