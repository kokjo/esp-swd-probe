use core::array::from_fn;

use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::gpio::Pull;

use esp_hal::gpio::Flex;
use esp_hal::peripheral::Peripheral;
use log::info;
use log::trace;
use thiserror::Error;

use crate::registers::ap;
use crate::registers::dp;
use crate::registers::dp::{RdBuff, Select};

pub struct Swd<'a> {
    pub swclk: Flex<'a>,
    pub swdio: Flex<'a>,
}

impl<'a> Swd<'a> {
    pub fn new(
        swclk: impl Peripheral<P = impl Into<AnyPin>> + 'a,
        swdio: impl Peripheral<P = impl Into<AnyPin>> + 'a,
    ) -> Self {
        let mut swclk = Flex::new(swclk);
        let mut swdio = Flex::new(swdio);
        swclk.set_as_output();
        swdio.set_as_output();
        Self { swclk, swdio }
    }

    pub async fn wait_clock(&self) {
        Timer::after_nanos(250).await
    }

    pub async fn swd_clock(&mut self, out: bool) -> bool {
        let bit = self.swdio.level().into();
        self.swdio.set_level(out.into());
        self.wait_clock().await;
        self.swclk.set_high();
        self.wait_clock().await;
        self.swclk.set_low();
        bit
    }

    pub async fn turnaround_host(&mut self) {
        trace!("Turnaround to HOST");
        self.swd_clock(false).await;
        self.swdio.set_as_output();
    }

    pub async fn turnaround_target(&mut self) {
        trace!("Turnaround to TARGET");
        self.swdio.set_as_input(Pull::None);
        self.swd_clock(false).await;
    }

    pub async fn recv_bits(&mut self, bits: &mut [bool]) {
        for bit in bits {
            *bit = self.swd_clock(false).await;
        }
    }

    pub async fn send_bits(&mut self, bits: &[bool]) {
        for &bit in bits {
            self.swd_clock(bit).await;
        }
    }

    pub async fn send_u32(&mut self, value: u32, length: usize) {
        let bits: [bool; u32::BITS as usize] = from_fn(|i| ((value >> i) & 1) == 1);
        self.send_bits(&bits[..length]).await
    }

    pub async fn send_u16(&mut self, value: u16, length: usize) {
        let bits: [bool; u16::BITS as usize] = from_fn(|i| ((value >> i) & 1) == 1);
        self.send_bits(&bits[..length]).await
    }
}

impl Swd<'_> {
    pub async fn line_reset(&mut self, low_clocks: usize) {
        self.send_bits(&[true; 50]).await;
        for _ in 0..low_clocks {
            self.swd_clock(false).await;
        }
    }

    pub async fn jtag_to_swd(&mut self) {
        self.send_bits(&[
            false, true, true, true, true, false, false, true, true, true, true, false, false,
            true, true, true,
        ])
        .await
    }

    pub async fn reset(&mut self) {
        trace!("Resetting SWD");
        self.swclk.set_as_output();
        self.swdio.set_as_output();
        self.line_reset(0).await;
        self.jtag_to_swd().await;
        self.line_reset(2).await;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum APnDP {
    DP,
    AP,
}

impl From<APnDP> for bool {
    fn from(val: APnDP) -> Self {
        match val {
            APnDP::DP => false,
            APnDP::AP => true,
        }
    }
}

impl From<bool> for APnDP {
    fn from(val: bool) -> Self {
        match val {
            false => APnDP::DP,
            true => APnDP::AP,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RnW {
    Read,
    Write,
}

impl From<RnW> for bool {
    fn from(value: RnW) -> Self {
        match value {
            RnW::Read => true,
            RnW::Write => false,
        }
    }
}

impl From<bool> for RnW {
    fn from(value: bool) -> Self {
        match value {
            true => RnW::Read,
            false => RnW::Write,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ack {
    Ok,
    Wait,
    Fault,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidAck;

impl TryFrom<[bool; 3]> for Ack {
    type Error = InvalidAck;

    fn try_from(value: [bool; 3]) -> Result<Self, Self::Error> {
        Ok(match value {
            [true, false, false] => Ack::Ok,
            [false, true, false] => Ack::Wait,
            [false, false, true] => Ack::Fault,
            _ => return Err(InvalidAck),
        })
    }
}

impl From<Ack> for [bool; 3] {
    fn from(value: Ack) -> Self {
        match value {
            Ack::Ok => [true, false, false],
            Ack::Wait => [false, true, false],
            Ack::Fault => [false, false, true],
        }
    }
}

#[derive(Debug, Copy, Clone, Error, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestError {
    #[error("Too many SWD wait acks")]
    Timeout,
    #[error("SWD Fault ack")]
    Fault,
    #[error("Invalid SWD ack, no reply?")]
    InvalidAck,
    #[error("Parity Error")]
    ParityError,
}

impl Swd<'_> {
    pub async fn send_request(&mut self, apndp: APnDP, rnw: RnW, a: [bool; 2]) {
        let apndp: bool = apndp.into();
        let rnw: bool = rnw.into();
        let parity = apndp ^ rnw ^ a[0] ^ a[1];
        let request = [true, apndp, rnw, a[0], a[1], parity, false, true];
        self.send_bits(&request).await;
    }

    pub async fn recv_ack(&mut self) -> Result<Ack, InvalidAck> {
        let mut ack = [false; 3];
        self.recv_bits(&mut ack).await;
        ack.try_into()
    }

    pub async fn read_request(&mut self, apndp: APnDP, a: [bool; 2]) -> Result<u32, RequestError> {
        let mut retries = 10;
        loop {
            self.send_request(apndp, RnW::Read, a).await;
            self.turnaround_target().await;
            match self
                .recv_ack()
                .await
                .map_err(|_| RequestError::InvalidAck)?
            {
                Ack::Ok => break,
                Ack::Wait => {
                    self.turnaround_host().await;
                    retries -= 1;
                    if retries == 0 {
                        return Err(RequestError::Timeout);
                    }
                    info!("Got Ack::Wait retrying {}", retries);
                    continue;
                }
                Ack::Fault => {
                    self.turnaround_host().await;
                    return Err(RequestError::Fault);
                }
            }
        }

        let mut bits = [false; 33];
        self.recv_bits(&mut bits).await;
        let parity = bits[..32].iter().fold(false, |p, b| p ^ b);
        if parity != bits[32] {
            return Err(RequestError::ParityError);
        }

        self.turnaround_host().await;

        Ok(bits[..u32::BITS as usize]
            .iter()
            .enumerate()
            .fold(0, |n, (i, b)| n | ((*b as u32) << (i as u32))))
    }

    pub async fn read_dp_register<Reg: dp::ReadRegister>(&mut self) -> Result<Reg, RequestError> {
        let reg = self.read_request(APnDP::DP, Reg::A).await.map(Into::into);
        trace!("Read DP register {:?} {:x?}", Reg::A, reg);
        reg
    }

    pub async fn write_request(
        &mut self,
        apndp: APnDP,
        a: [bool; 2],
        value: u32,
    ) -> Result<(), RequestError> {
        let mut retries = 10;
        loop {
            self.send_request(apndp, RnW::Write, a).await;
            self.turnaround_target().await;
            match self.recv_ack().await {
                Ok(Ack::Ok) => break,
                Ok(Ack::Wait) => {
                    self.turnaround_host().await;
                    retries -= 1;
                    if retries == 0 {
                        return Err(RequestError::Timeout);
                    }
                    trace!("Got Ack::Wait retrying {}", retries);
                    continue;
                }
                Ok(Ack::Fault) => {
                    self.turnaround_host().await;
                    return Err(RequestError::Fault);
                }
                Err(InvalidAck) => {
                    self.turnaround_host().await;
                    return Err(RequestError::InvalidAck);
                }
            }
        }
        self.turnaround_host().await;

        let bits: [bool; 32] = from_fn(|i| (value >> i) & 1 == 1);
        self.send_bits(&bits).await;
        let parity = bits.iter().fold(false, |p, b| p ^ b);
        self.send_bits(&[parity]).await;

        Ok(())
    }

    pub async fn write_dp_register<Reg: dp::WriteRegister>(
        &mut self,
        reg: Reg,
    ) -> Result<(), RequestError> {
        trace!("Writing DP register {:?} {:x?}", Reg::A, &reg);
        self.write_request(APnDP::DP, Reg::A, reg.into()).await
    }

    pub async fn read_ap(&mut self, ap: u8, addr: u8) -> Result<u32, RequestError> {
        self.write_dp_register(Select::default().set_apsel(ap).set_apbanksel(addr >> 4))
            .await?;
        self.read_request(APnDP::AP, [addr & 0x04 == 0x04, addr & 0x08 == 0x08])
            .await?;
        let value = self.read_dp_register::<RdBuff>().await?.0;
        trace!("Reading AP register {:02x}:{:02x}: {:08x}", ap, addr, value);
        Ok(value)
    }

    pub async fn read_ap_register<Reg: ap::ReadRegister>(
        &mut self,
        ap: u8,
    ) -> Result<Reg, RequestError> {
        let value = self.read_ap(ap, Reg::ADDRESS).await.map(Into::into)?;
        trace!(
            "Reading AP Register {:02}:{:02x}: {:?}",
            ap,
            Reg::ADDRESS,
            value
        );
        Ok(value)
    }

    pub async fn write_ap(&mut self, ap: u8, addr: u8, value: u32) -> Result<(), RequestError> {
        trace!("Writing AP register {:02x}:{:02x}: {:08x}", ap, addr, value);
        self.write_dp_register(Select::default().set_apsel(ap).set_apbanksel(addr >> 4))
            .await?;
        self.write_request(APnDP::AP, [addr & 0x04 == 0x04, addr & 0x08 == 0x08], value)
            .await?;
        Ok(())
    }

    pub async fn write_ap_register<Reg: ap::WriteRegister>(
        &mut self,
        ap: u8,
        reg: Reg,
    ) -> Result<(), RequestError> {
        trace!(
            "Writing AP register {:02x}:{:02x}: {:?}",
            ap,
            Reg::ADDRESS,
            reg
        );
        self.write_ap(ap, Reg::ADDRESS, reg.into()).await
    }
}
