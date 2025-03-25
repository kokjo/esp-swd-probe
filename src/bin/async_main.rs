#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_swd_probe::registers::ap::{memap, Idr};
use esp_swd_probe::registers::dp::{CtrlStat, Idcode};
use esp_swd_probe::swd::{RequestError, Swd};

use log::info;

extern crate alloc;

pub async fn test_swd(swd: &mut Swd<'_>) -> Result<(), RequestError> {
    swd.swd_clock(false).await;
    Timer::after_nanos(1000).await;

    swd.reset().await;

    info!("idcode = {:x?}", swd.read_dp_register::<Idcode>().await);

    let _ = swd.write_dp_register(CtrlStat::default()).await;

    info!("status = {:x?}", swd.read_dp_register::<CtrlStat>().await);

    let _ = swd
        .write_dp_register(
            CtrlStat::default()
                .set_csyspwrupreq(true)
                .set_cdbgpwrupreq(true)
                .set_cdbgrstreq(true),
        )
        .await;

    let _ = swd.write_dp_register(
        CtrlStat::default()
                .set_csyspwrupreq(true)
                .set_cdbgpwrupreq(true)
        )
        .await;

    const AP: u8 = 0;

    let idr = swd.read_ap_register::<Idr>(AP).await?;
    info!("IDR = {:x?}", idr);

    let base = swd.read_ap_register::<memap::Base>(AP).await?;
    info!("BASE = {:x?}", base);

    let _ = swd.write_ap(AP, 0x04, base.address() + 0xff0).await;

    for i in 0..4 {
        info!("{:08x?}", swd.read_ap(AP, 0x10 + 4 * i).await);
    }

    info!("status = {:x?}", swd.read_dp_register::<CtrlStat>().await);

    let _ = swd.write_dp_register(CtrlStat::default()).await;

    Ok(())
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

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let _ = spawner;

    let mut swd = Swd::new(peripherals.GPIO21, peripherals.GPIO20);

    loop {
        let _ = test_swd(&mut swd).await;
        Timer::after(Duration::from_secs(1)).await;
    }
}
