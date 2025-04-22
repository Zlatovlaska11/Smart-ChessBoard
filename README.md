
# ♟️ Smart Chess Board

A smart chess board system with a Rust-based embedded client and a Go-powered server for move processing, game visualization, and interaction.

## 📦 Project Overview

This project aims to bring together embedded systems, real-time communication, and chess logic to create a physical chess board that:

- Detects and validates moves in real-time
- Displays the current game state via a web interface
- Can integrate with chess engines or online play in the future

---

## 🧱 Architecture

```
+----------------------------+       +---------------------------+
|     Rust Client (Board)   | --->  |      Go Server (Backend)  |
| - Runs on RPi / Arduino   |       | - Validates moves         |
| - Reads sensor input      |       | - Stores game state       |
| - Sends moves via HTTP/gRPC|       | - Serves web UI / API     |
+----------------------------+       +---------------------------+
```

---

## ✨ Features (Planned)

- ✅ Move detection from the physical board
- ✅ Server-side validation and board state tracking
- ✅ Web UI to visualize the current board and move history
- 🚧 Support for UCI chess engines (e.g. Stockfish)
- 🚧 Multiplayer / remote play via the smart board
- 🚧 Puzzle / training mode with guided LEDs


---

## 🛠 Tech Stack

- **Rust** — for embedded systems (safe, fast, low-level control)
- **Go** — for the backend server (simple, performant, easy concurrency)
- **gRPC / REST** — for communication between board and server

---

## 🚀 Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/smart-chess-board.git
   ```
  
3. Run the client (simulation for now):
   ```bash
   cd client
   cargo run
   ```

4. Visit the web dashboard at `http://localhost:8080`

---

## 🧪 Development Notes

- Hardware integration will use GPIOs or serial via Rust on Raspberry Pi or Arduino
- Until then, the client simulates moves
---

## 📌 TODO

- [ ] Design message protocol between client and server
- [ ] Build board state manager in Go
- [ ] Create simulation for client moves
- [ ] UI for displaying board & move history
- [ ] Hardware integration phase

---

## 📖 License

MIT – do whatever you want ✨

---

## 🤝 Contributions

Suggestions, improvements, and crazy feature ideas are always welcome! Open an issue or start a discussion.
