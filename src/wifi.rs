use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_hal::{peripheral::Peripheral, timer::timg::TimerGroup};
use esp_wifi::{
    wifi::{ClientConfiguration, Configuration, WifiController, WifiEvent, WifiStaDevice},
    EspWifiController, EspWifiRngSource,
};
use log::info;

use crate::mk_static;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub async fn start_wifi_sta(
    spawner: &Spawner,
    rng: impl EspWifiRngSource,
    timg: esp_hal::peripherals::TIMG0,
    radio_clk: impl Peripheral<P = esp_hal::peripherals::RADIO_CLK> + 'static,
    wifi: impl Peripheral<P = esp_hal::peripherals::WIFI> + 'static,
) -> esp_wifi::wifi::WifiDevice<'static, WifiStaDevice> {
    // Setup and spawn wifi stack
    let timg0 = TimerGroup::new(timg);

    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timg0.timer0, rng, radio_clk).unwrap()
    );

    let (wifi_sta, wifi_ctrl) =
        esp_wifi::wifi::new_with_mode(esp_wifi_ctrl, wifi, WifiStaDevice).unwrap();
    spawner.must_spawn(wifi_ctrl_task(wifi_ctrl));

    wifi_sta
}

#[embassy_executor::task]
async fn wifi_ctrl_task(mut wifi_ctrl: WifiController<'static>) {
    wifi_ctrl
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
            ..Default::default()
        }))
        .unwrap();

    wifi_ctrl.start_async().await.unwrap();

    loop {
        info!("Connecting to {}", SSID);
        match wifi_ctrl.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => info!("Failed to connect to wifi: {e:?}"),
        }
        if wifi_ctrl.is_connected().map_err(|_| ()) == Ok(true) {
            info!("Waiting for wifi to disconnect");
            wifi_ctrl.wait_for_event(WifiEvent::StaDisconnected).await;
        }
        Timer::after_secs(1).await
    }
}
