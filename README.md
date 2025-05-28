## Installation:
1. ✅ **Use as a dependency** (via `git` in `Cargo.toml`)
2. ✅ **Clone and build locally** (develop/test directly inside the SDK repo)

---

## 📦 `dop-sdk`

A hybrid **Rust + Node.js SDK** that wraps a powerful backend engine implemented in TypeScript, exposing wallet, encryption, decryption, and transaction APIs via a local Node.js service.

This SDK automatically starts the Node.js engine and communicates with it via HTTP.

---

## 📁 Project Structure

```
dop-sdk/
├── src/                 # Rust SDK
│   └── lib.rs
├── ts-lib/              # Node.js engine (Express API)
│   ├── src/
│   ├── dist/
│   └── package.json
├── build.rs             # Auto-builds Node backend
├── build.sh             # Manual build script (optional)
├── Cargo.toml
└── README.md
```

---

## 🚀 Features

- 📡 Automatically starts Node.js engine from Rust
- 🔐 Wallet creation, encryption, decryption, transfer
- ⚡ Uses `reqwest` for HTTP communication
- 🔁 Auto-builds Node backend during `cargo build`
- 🧪 Easy testing and automatic teardown via `Drop`

---

## ✅ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.70
- [Node.js](https://nodejs.org/) + `npm`
- TypeScript is optional (`tsc` is run via `npm run build`)

---

## 🛠 Usage

You can use this SDK in **two different ways**:

---

### ✅ Option 1: Use as a Git Dependency (Recommended for consumers)

Add this to your app's `Cargo.toml`:

```toml
[dependencies]
dop = { git = "https://github.com/VAR-META-Tech/dop-rust", branch = "main" }
```

Then use it in your code:

```rust
use dop::DopClient;

#[tokio::main]
async fn main() {
    let mut client = DopClient::new();
    client.start();
    client.wait_for_api_ready().await;

    // use client...
}
```

> 🛠 The SDK will automatically run `npm install` and `npm run build` (via `build.rs`) when you compile with `cargo build`.

---

### 🧪 Option 2: Clone Locally (for development or contributions)

```bash
git clone https://github.com/VAR-META-Tech/dop-rust.git
cd dop-rust
```

#### Option A: Use `build.rs` (automatic build)

```bash
cargo build
```

Rust will:
- Run `npm install`
- Run `npm run build`
- Compile the SDK

#### Option B: Manually build the Node backend

```bash
sh build.sh
cargo build
```

---

## 🧪 Running Tests

```bash
cargo test
```

Make sure the Node backend is built (automatically or manually).

---

## 📄 License

MIT or Apache 2.0 — choose one and update `Cargo.toml`.

