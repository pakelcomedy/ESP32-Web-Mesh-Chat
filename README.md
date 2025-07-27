ESP32-Web-Mesh-Chat/
├── Cargo.toml
├── memory.x
├── .cargo/
│   └── config.toml
├── src/
│   ├── main.rs          # Init Wi-Fi, web server, esp-now, handler
│   ├── wifi.rs
│   ├── esp_now.rs       # Send/recv ESP-NOW
│   ├── http.rs          # Serve HTML + handle REST API
│   └── chat.rs          # Logic untuk menyimpan chat, handle buffer
└── website/             # UI untuk chatting
    ├── index.html       # Halaman chat
    ├── style.css
    └── script.js        # AJAX POST/GET message
