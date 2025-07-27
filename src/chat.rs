// src/chat.rs

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: u64,
}

// shared in‑memory chat log (keeps last 100 messages)
static CHAT_LOG: Lazy<Mutex<Vec<ChatMessage>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

/// Add a new message to the log (drops oldest if >100)
pub fn add_message(msg: ChatMessage) {
    let mut log = CHAT_LOG.lock().unwrap();
    log.push(msg);
    if log.len() > 100 {
        log.remove(0);
    }
}

/// Get a snapshot of all stored messages
pub fn get_messages() -> Vec<ChatMessage> {
    CHAT_LOG.lock().unwrap().clone()
}
