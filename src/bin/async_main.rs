#![no_std]
#![no_main]

use alloc::vec::Vec;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_swd_probe::net::{start_net, wait_for_dhcp, wait_for_link};
use esp_swd_probe::registers::ap::Idr;
use esp_swd_probe::registers::dp::{CtrlStat, Idcode};
use esp_swd_probe::swd::{APnDP, RequestError, Swd};

use esp_swd_probe::wifi;
use log::{debug, info};
use thiserror::Error;

extern crate alloc;

pub async fn test_swd(swd: &mut Swd<'_>) -> Result<(), RequestError> {
    swd.swd_clock(false).await;
    Timer::after_nanos(1000).await;

    swd.reset().await;

    info!("idcode = {:x?}", swd.read_dp_register::<Idcode>().await?);

    swd.write_dp_register(CtrlStat::default()).await?;

    swd.modify_dp_register::<CtrlStat>(|reg| {
        info!("ctrlstat = {:x?}", reg);
        reg
            .set_csyspwrupreq(true)
            .set_cdbgpwrupreq(true)
            .set_cdbgrstreq(true)
    }).await?;

    swd.modify_dp_register::<CtrlStat>(|reg| {
        info!("ctrlstat = {:x?}", reg);
        reg.set_cdbgrstreq(false)
    }).await?;

    const AP: u8 = 0;

    let idr = swd.read_ap_register::<Idr>(AP).await?;
    info!("IDR = {:x?}", idr);

    let mut memap = swd.memap(AP);
    let base = memap.base().await?;
    info!("BASE = {:x?}", base);
    for i in (0x0ff0..0x1000).step_by(4) {
        let address = base.address() + i;
        let value = memap.read_32(address).await?;
        info!("{:#010x} = {:#010x}", address, value);
    }

    swd.write_dp_register(CtrlStat::default()).await?;
    info!("status = {:x?}", swd.read_dp_register::<CtrlStat>().await);

    Ok(())
}

#[derive(Debug, Clone)]
pub enum Command {
    ReadDp(u8),
    WriteDp(u8, u32),
    ReadAp(u8),
    WriteAp(u8, u32),
    SwjSequence(u8, u64),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Empty command")]
    EmptyCommand,
    #[error("Unknown command")]
    UnknownCommand,
    #[error("Command too short")]
    TooShort,
}

impl TryFrom<&[u8]> for Command {
    type Error = CommandError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (&cmd, data) = data.split_first().ok_or(CommandError::EmptyCommand)?;
        match cmd {
            0x00 => {
                if data.len() < 1 {
                    return Err(CommandError::TooShort);
                }
                Ok(Command::ReadDp(data[0]))
            },
            0x01 => {
                if data.len() < 5 {
                    return Err(CommandError::TooShort);
                }
                Ok(Command::WriteDp(data[0], u32::from_be_bytes(data[1..].try_into().unwrap())))
            },
            0x02 => {
                if data.len() < 1 {
                    return Err(CommandError::TooShort);
                }
                Ok(Command::ReadAp(data[0]))
            },
            0x03 => {
                if data.len() < 5 {
                    return Err(CommandError::TooShort);
                }
                Ok(Command::WriteAp(data[0], u32::from_be_bytes(data[1..].try_into().unwrap())))
            },
            0x04 => {
                if data.len() < 9 {
                    return Err(CommandError::TooShort);
                }
                Ok(Command::SwjSequence(data[0], u64::from_be_bytes(data[1..].try_into().unwrap())))
            }
            _ => Err(CommandError::UnknownCommand)
        }
    }
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Command error: {0}")]
    CommandError(#[from] CommandError),
    #[error("Reply too big")]
    ReplyTooBig,
    #[error("EOF")]
    EOF,
}

pub async fn recv_message(sock: &mut TcpSocket<'_>) -> Result<Vec<u8>, ProtocolError> {
    let mut size = [0u8; 1];

    sock.read_exact(&mut size).await.map_err(|_| ProtocolError::EOF)?;
    let mut msg = Vec::with_capacity(size[0] as usize);
    for _ in 0..size[0] {
        msg.push(0x00);
    }
    sock.read_exact(&mut msg).await.map_err(|_| ProtocolError::EOF)?;
    Ok(msg)
}

pub async fn send_message(sock: &mut TcpSocket<'_>, msg: &[u8]) -> Result<(), ProtocolError> {
    let size: u8 = msg.len().try_into().map_err(|_|ProtocolError::ReplyTooBig)?;
    sock.write_all(&[size]).await.map_err(|_| ProtocolError::EOF)?;
    sock.write_all(msg).await.map_err(|_| ProtocolError::EOF)?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Reply {
    Read(Result<u32, RequestError>),
    Write(Result<(), RequestError>),
}

impl From<Reply> for Vec<u8> {
    fn from(value: Reply) -> Self {
        let mut msg = Vec::new();
        match value {
            Reply::Read(Ok(value)) => {
                msg.push(0x00);
                msg.extend(value.to_be_bytes())
            },
            Reply::Read(Err(err)) => {
                msg.push(err.into());
            }
            Reply::Write(Ok(())) => {
                msg.push(0x00);
            },
            Reply::Write(Err(err)) => {
                msg.push(err.into())
            }
        }
        msg
    }
}

pub fn a_to_bits(a: u8) -> [bool; 2] {
    [a & 0x04 == 0x04, a & 0x08 == 0x08]
}

pub async fn handle_connection(sock: &mut TcpSocket<'_>, swd: &mut Swd<'_>) -> Result<(), ProtocolError> {
    loop {
        let msg = recv_message(sock).await?;
        let cmd: Command = (&msg[..]).try_into()?;
        debug!("Command: {:x?}", cmd);
        let reply: Reply = match cmd {
            Command::ReadDp(a) => {
                Reply::Read(swd.read_request(APnDP::DP, a_to_bits(a)).await).into()
            },
            Command::WriteDp(a, value) => {
                Reply::Write(swd.write_request(APnDP::DP, a_to_bits(a), value).await).into()
            },
            Command::ReadAp(a) => {
                Reply::Read(swd.read_selected_ap(a).await).into()
            },
            Command::WriteAp(a, value) => {
                Reply::Write(swd.write_selected_ap(a, value).await).into()
            },
            Command::SwjSequence(bit_len, bits) => {
                swd.swj_sequence(bit_len, bits).await;
                Reply::Write(Ok(())).into()
            },
        };
        debug!("Reply: {:x?}", reply);
        let msg: Vec<u8> = reply.into();
        send_message(sock, &msg).await?;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);

    let wifi_sta = wifi::start_wifi_sta(&spawner, rng, peripherals.TIMG0, peripherals.RADIO_CLK, peripherals.WIFI).await;
    let stack = start_net(spawner, rng, wifi_sta);
    wait_for_link(stack).await;
    wait_for_dhcp(stack).await;

    let mut txbuf = [0u8; 4096];
    let mut rxbuf = [0u8; 4096];

    let mut swd = Swd::new(peripherals.GPIO21, peripherals.GPIO20);

    loop {
        let mut socket = TcpSocket::new(stack, &mut rxbuf, &mut txbuf);
        info!("Waiting for connection!");
        match socket.accept(1337).await {
            Ok(()) => {
                info!("Accepted connection from {}", socket.remote_endpoint().unwrap());
                let res = handle_connection(&mut socket, &mut swd).await;
                info!("Debug Session done: {:?}", res);

            },
            Err(err) => {
                info!("Failed to accept on socket: {:?}", err)
            },
        }
    }
}
