# 📻 faf-radio-rust

**Radio Protocol client for Rust - AI Context Broadcasting**

> Day 2 of the Triple Launch: Rust Edition

[![Rust](https://img.shields.io/badge/rust-1.85+-orange)](https://rust-lang.org)
[![Edition](https://img.shields.io/badge/edition-2024-blue)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![Protocol](https://img.shields.io/badge/protocol-Radio%20v1.0-orange)](https://mcpaas.live/beacon)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

---

## The Big Idea

Instead of re-sending AI context every session (expensive, slow, drift-prone), **broadcast once** and let everyone tune in.

```
❌ Traditional (The Letter Model)
You → Claude (send 50KB)
You → Grok (send 50KB again)
You → Gemini (send 50KB again)
Cost: 150KB, 3 API calls, potential drift

✅ Radio Protocol (The Broadcasting Model)
You → Radio (broadcast 50KB once)
Claude, Grok, Gemini → Tune to 91.0 FM
Cost: 50KB, 1 broadcast, zero drift
```

**99% cost reduction. Zero context drift. Infinite listeners.**

---

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
faf-radio-rust = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Usage

```rust
use faf_radio_rust::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Radio Protocol server
    let url = "wss://faf-beacon.wolfejam2020.workers.dev/radio";
    let mut radio = RadioClient::new(RadioConfig::new(url));

    radio.connect().await?;

    // Tune to a frequency
    radio.tune(vec!["91.0".to_string()]).await?;

    // Listen for broadcasts...
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    radio.disconnect().await?;
    Ok(())
}
```

---

## Architecture

**Listen-Only Clients**

Rust clients tune to frequencies and receive broadcasts. Broadcasts are triggered server-side when context updates via HTTP POST.

```
┌─────────────────────────────────────────────┐
│           Radio Protocol Flow               │
├─────────────────────────────────────────────┤
│                                             │
│  1. Client connects via WebSocket           │
│  2. Client tunes to frequency (91.0 FM)     │
│  3. Server broadcasts when soul updates     │
│  4. All tuned clients receive instantly     │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Available Frequencies

| Frequency | Soul | Purpose |
|-----------|------|---------|
| 91.0 FM | nelly | Personal context (eternal memory demo) |
| 92.5 FM | faf | Project updates |
| 94.7 FM | wolfejam | Team context |

---

## Examples

### Basic Example

```bash
cargo run --example basic
```

See `examples/basic.rs` for full code.

### Multi-AI Example

```bash
cargo run --example multi_ai
```

Demonstrates 3 AIs (Claude, Grok, Gemini) tuned to the same frequency, all receiving broadcasts simultaneously.

---

## Technical Stack

- **Runtime:** Tokio (async/await)
- **WebSocket:** tokio-tungstenite v0.28
- **Rust Edition:** 2024
- **Serialization:** serde + serde_json

**Features:**
- Auto-reconnection with exponential backoff
- Heartbeat mechanism (ping/pong every 30s)
- Connection state management
- Frequency validation (40.0-108.0 FM)
- Type-safe protocol messages

---

## Triple Launch Strategy

| Day | Language | Status |
|-----|----------|--------|
| **Day 1** | Bun/TypeScript | ✅ Complete ([faf-radio-bun](https://github.com/Wolfe-Jam/faf-radio-bun)) |
| **Day 2** | Rust | 🔜 In Progress (this repo) |
| **Day 3** | Zig WASM (2.7KB) | 🔜 Pending |

---

## Server

**Production:** `wss://faf-beacon.wolfejam2020.workers.dev/radio`

Deployed on Cloudflare Workers (300+ edge locations). See [mcpaas-beacon](https://github.com/Wolfe-Jam/mcpaas-beacon) for server implementation.

---

## 📋 Project Context (6Ws)

**1W (WHO):** Rust developers building AI-powered applications

**2W (WHAT):** 99% cost reduction via broadcast-once context distribution (Radio Protocol)

**3W (WHERE):** Rust applications, CLI tools, servers

**4W (WHY):** Day 2 of Triple Launch Strategy - prove Radio Protocol works in Rust

**5W (WHEN):** Day 2 (February 2026) - part of Bun→Rust→Zig championship rollout

**6W (HOW):** WebSocket client library with auto-reconnect, frequency tuning, and heartbeat

---

## License

MIT © wolfejam

---

*Radio is back. Loved way more than REST.* 📻
