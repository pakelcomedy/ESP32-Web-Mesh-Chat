# 🔌 ESP32 Web Mesh Chat

**Offline Web-Based Chat System using ESP-NOW and Wi-Fi**

---

## 📦 Project Structure

```
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
```

---

## 🧠 How It Works

ESP32 acts as a **local web server** and **mesh radio node** using `ESP-NOW` for peer-to-peer communication. Messages are typed from a browser connected to ESP32's Wi-Fi hotspot. Each message is:

1. **Sent from Browser to ESP32** (via REST API).
2. **Stored locally** in buffer.
3. **Broadcasted to all other ESP32 nodes** via ESP-NOW.
4. **Displayed in chat UI** by polling recent messages every second.

```text
+-------------+      ESP-NOW      +-------------+
|   ESP32 A   | <---------------> |   ESP32 B   |
|  WebServer  |                   |  WebServer  |
+-------------+                   +-------------+
       ^                                 ^
       | Wi-Fi AP                        | USB Serial
       |                                 |
+------------+                   +------------+
|   Browser  |                   |   Browser  |
+------------+                   +------------+
```

✅ All nodes store and show the same messages.

✅ New ESP32s can discover and join automatically.

✅ Messages limited to 50 chars (configurable).

---

## 📡 Offline First — No Internet Required!

This project is **completely offline**:

* No router or external access point needed.
* ESP32 creates its own Wi-Fi AP.
* Communication between devices is 100% over `ESP-NOW`, a low-power, long-range protocol by Espressif.

---

## ⚙️ Features

* 🧠 Peer-to-peer mesh via ESP-NOW
* 🌐 Built-in web server with HTML/JS UI
* ✉️ Real-time chat with 1-second polling
* 📁 Messages stored in RAM (can be extended)
* 📶 Works in remote/off-grid situations

---

## 🛠️ Getting Started

1. Flash this project to multiple ESP32s.
2. Connect your phone/laptop to ESP32's Wi-Fi hotspot.
3. Open browser → go to `http://192.168.4.1`
4. Start chatting — messages will sync across all ESP32s nearby.

---

## 📷 UI Preview

![ESP32 Mesh Chat Web UI](https://github.com/user-attachments/assets/d5357746-6095-4a62-bc25-97e6638430cf)

---

## 🔒 Notes

* ESP-NOW max payload is 250 bytes. Limit messages to \~100 bytes for safety.
* You can connect OLED or LCD for local display.
* Message buffer is in-memory (RAM); can be saved to SPIFFS if needed.

---

## 📚 License

MIT License — feel free to fork, use, and modify.
