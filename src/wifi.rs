// src/wifi.rs

use anyhow::Result;
use embedded_svc::wifi::{
    AccessPointConfiguration, Configuration, Wifi,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
    netif::EspNetifIpInfo,
};
use log::info;

/// Configure ESP32 as a SoftAP so browsers can connect to the chat UI.
pub struct WifiAp {
    wifi: EspWifi,
}

impl WifiAp {
    /// Initialize SoftAP with given SSID/password.
    pub fn init(
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
        ssid: &str,
        password: &str,
    ) -> Result<Self> {
        let mut wifi = EspWifi::new_default(sysloop, Some(nvs))?;
        let ap_conf = AccessPointConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            channel: 1,
            ssid_hidden: false,
            max_connections: 8,
            ..Default::default()
        };
        wifi.set_configuration(&Configuration::Ap(ap_conf))?;
        wifi.start()?;
        let ip = wifi.ap_netif().get_ip_info()?.ip;
        info!("🌐 SoftAP `{}` started, IP = {}", ssid, ip);
        Ok(WifiAp { wifi })
    }

    /// Returns the AP IP so you can display e.g. "Connect to http://192.168.4.1"
    pub fn ip_address(&self) -> Result<EspNetifIpInfo<'_>> {
        let info = self.wifi.ap_netif().get_ip_info()?;
        Ok(info)
    }
}
