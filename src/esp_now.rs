// src/esp_now.rs

use anyhow::Result;
use esp_idf_svc::esp_now::{EspNow, PeerInfo};
use heapless::Vec;
use log::info;
use crate::chat::{add_message, ChatMessage};

/// Mesh layer: broadcast & receive via ESP‑NOW
pub struct Mesh {
    esp_now: EspNow,
}

impl Mesh {
    /// Initialize ESP‑NOW and set up receive callback.
    pub fn init() -> Result<Self> {
        let mut esp_now = EspNow::new_default()?;
        // Broadcast discovery: allow any peer
        esp_now.set_self_role(esp_idf_sys::esp_now_role_t_ESP_NOW_ROLE_COMBO)?;
        // Register receive callback
        esp_now.register_recv_cb(move |peer: &PeerInfo, data: &[u8]| {
            if let Ok(text) = core::str::from_utf8(data) {
                let msg = ChatMessage {
                    sender: format!("{:02X?}", peer.mac_addr),
                    message: text.to_string(),
                    timestamp: esp_idf_svc::timer::EspTimer::now().ticks(),
                };
                add_message(msg);
                info!("📥 Received from {:?}: {}", peer.mac_addr, text);
            }
        })?;
        Ok(Mesh { esp_now })
    }

    /// Broadcast a chat line (under 250 bytes).
    pub fn broadcast(&mut self, text: &str) -> Result<()> {
        let data = text.as_bytes();
        // 0xFF.. broadcast address
        let mut bcast: Vec<u8, 6> = Vec::default();
        bcast.extend_from_slice(&[0xFF; 6]).unwrap();
        self.esp_now.send(&bcast, data)?;
        info!("📤 Broadcast: {}", text);
        Ok(())
    }
}
