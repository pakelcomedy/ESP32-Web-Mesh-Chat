// src/main.rs

#![no_std]
#![no_main]

use esp_idf_sys as _;                     // link ESP‑IDF
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration as HttpConfig, EspHttpServer},
    log::EspLogger,
    nvs::EspDefaultNvsPartition,
};
use log::*;
use anyhow::Result;
use core::panic::PanicInfo;
use std::{thread, time::Duration};

mod wifi;
mod esp_now;
mod http;
mod chat;

use wifi::WifiAp;
use esp_now::Mesh;
use http::HttpHandler;

/// Required for `#![no_std]`
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    EspLogger::println(format_args!("Panic: {:?}", info)).unwrap();
    loop {}
}

#[entry]
fn main() -> Result<()> {
    // initialize logger
    EspLogger::initialize_default();
    info!("🚀 ESP32 Mesh Chat starting…");

    // system event loop
    let sysloop = EspSystemEventLoop::take()?;

    // 1. start SoftAP for web UI
    let ap = WifiAp::init(
        sysloop.clone(),
        EspDefaultNvsPartition::take()?,
        "ESP32_Chat",
        "chat1234",
    )?;
    let ap_ip = ap.ip_address()?.ip;
    info!("✅ SoftAP up: connect to http://{}", ap_ip);

    // 2. init ESP-NOW mesh
    let mut mesh = Mesh::init()?;

    // 3. start HTTP server
    let server = EspHttpServer::new(&HttpConfig::default())?;
    let http = HttpHandler::new(server, mesh);
    http.mount()?;
    info!("🌐 Web UI mounted");

    // 4. run forever
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
