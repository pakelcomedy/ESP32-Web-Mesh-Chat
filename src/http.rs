// src/http.rs

use anyhow::{Context, Result};
use esp_idf_svc::http::server::{EspHttpServer, Response};
use embedded_svc::http::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{chat::{get_messages, ChatMessage}, mesh::Mesh};

/// Serve chat UI and JSON API
pub struct HttpHandler {
    server: EspHttpServer,
    mesh: Mesh,
}

#[derive(Deserialize)]
struct SendRequest {
    sender: String,
    message: String,
}

#[derive(Serialize)]
struct MsgOut<'a> {
    sender: &'a str,
    message: &'a str,
    timestamp: u64,
}

impl HttpHandler {
    pub fn new(server: EspHttpServer, mesh: Mesh) -> Self {
        HttpHandler { server, mesh }
    }

    /// Mount routes:
    /// • GET  /            → serve index.html from SPIFFS
    /// • GET  /messages    → return JSON list of ChatMessage
    /// • POST /send        → accept JSON, broadcast, store
    pub fn mount(self) -> Result<()> {
        let spiffs = "/spiffs"; // where you mounted SPIFFS

        // Static UI
        self.server.fn_handler("/*", Method::Get, move |req| {
            let path = req.uri();
            let rel = if path == "/" { "/index.html" } else { path };
            let full = format!("{}{}", spiffs, rel);
            let data = std::fs::read(&full)
                .context(format!("Reading {}", full))?;
            let ctype = if full.ends_with(".html") {
                "text/html"
            } else if full.ends_with(".css") {
                "text/css"
            } else {
                "application/javascript"
            };
            let mut res = Response::new(200);
            res.set_header("Content-Type", ctype);
            res.send_data(&data);
            Ok(res)
        })?;

        // List messages
        self.server.fn_handler("/messages", Method::Get, |_req| {
            let msgs = get_messages();
            let out: Vec<MsgOut> = msgs.iter()
                .map(|m| MsgOut { sender: &m.sender, message: &m.message, timestamp: m.timestamp })
                .collect();
            let body = serde_json::to_string(&out).unwrap();
            let mut res = Response::new(200);
            res.set_header("Content-Type", "application/json");
            res.send_str(&body);
            Ok(res)
        })?;

        // Send a new message
        self.server.fn_handler("/send", Method::Post, move |req| {
            let mut buf = Vec::new();
            req.into_reader().read_to_end(&mut buf).unwrap();
            let sr: SendRequest = serde_json::from_slice(&buf).context("Invalid JSON")?;
            // broadcast via ESP-NOW
            self.mesh.broadcast(&format!("{}: {}", sr.sender, sr.message))?;
            // store locally
            let msg = ChatMessage {
                sender: sr.sender,
                message: sr.message,
                timestamp: esp_idf_svc::timer::EspTimer::now().ticks(),
            };
            crate::chat::add_message(msg);
            let mut res = Response::new(201);
            res.send_str("OK");
            Ok(res)
        })?;

        Ok(())
    }
}
