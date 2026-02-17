# 🛡️ stdya-blockchain
A high-performance, deterministic consensus engine built in Rust.

License: MIT https://opensource.org
Rust https://www.rust-lang.org

## 📖 Overview
`stdya-blockchain` is a lightweight [Byzantine Fault Tolerant (BFT)](https://github.com) consensus engine. It focuses on 'determinism' and 'safety', specifically avoiding floating-point arithmetic to ensure consistent state across all network nodes.

## ✨ Features
- Deterministic Math: Custom `math.rs` module using fixed-point arithmetic [u128] for safe token calculations.
- View-Change Protocol: Automated leader rotation and timeout-based election logic.
- Cryptographic Security: Uses [Ed25519-Dalek](https://docs.rs) for robust identity and signing.
- Real-time Monitoring: Built-in colored logging for tracking consensus health.

## 🚀 Getting Started
### Prerequisites
- [Rust & Cargo](https://doc.rust-lang.org) installed.

### Installation

git clone https://github.com/hex2gdb/colya.git
cd stdya-blockchain
cargo build --release

