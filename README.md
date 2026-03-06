# faf-radio-rust

**Radio Protocol client for Rust.** Broadcast AI context once, every tool receives.

Client SDK for [MCPaaS](https://mcpaas.live) — persistent AI context infrastructure.

## Install

```toml
[dependencies]
faf-radio-rust = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use faf_radio_rust::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut radio = RadioClient::new(
        RadioConfig::new("wss://faf-beacon.wolfejam2020.workers.dev/radio")
    );

    radio.connect().await?;
    radio.tune(vec!["91.0".to_string()]).await?;

    // Context arrives from any AI, any session
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    radio.disconnect().await?;
    Ok(())
}
```

## How It Works

MCPaaS uses the **Radio Protocol** — broadcast once, every AI receives.

```
You → Broadcast to 91.0 FM (send once)
  Claude  ← tuned in
  Grok    ← tuned in
  Gemini  ← tuned in
= 1x cost, instant, zero drift
```

## Multi-AI Example

```rust
use faf_radio_rust::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://faf-beacon.wolfejam2020.workers.dev/radio";

    let mut claude = RadioClient::new(RadioConfig::new(url));
    let mut grok   = RadioClient::new(RadioConfig::new(url));
    let mut gemini = RadioClient::new(RadioConfig::new(url));

    claude.connect().await?;
    grok.connect().await?;
    gemini.connect().await?;

    // All three tune to the same frequency
    for radio in [&mut claude, &mut grok, &mut gemini] {
        radio.tune(vec!["91.0".to_string()]).await?;
    }

    // All three AIs now share the same context, in real time
    Ok(())
}
```

## API

| Method | Description |
|--------|-------------|
| `connect().await` | Connect to MCPaaS |
| `tune(frequencies).await` | Subscribe to frequencies |
| `untune(frequencies).await` | Unsubscribe |
| `disconnect().await` | Disconnect |
| `state().await` | Get connection state |
| `validate_frequencies(&freqs)` | Validate range (40.0-108.0 FM) |

### Features

- Auto-reconnection with exponential backoff
- Heartbeat (ping/pong every 30s)
- Frequency validation (40.0-108.0 FM)
- Type-safe protocol messages
- Tokio async/await

## Testing

**46/46 passing** — WJTTC Championship-Grade:

| Tier | Tests | What |
|------|-------|------|
| T1 BRAKES | 14 | Security — injection, boundary, validation |
| T2 ENGINE | 18 | Core — state, config, serialization |
| T3 AERO | 12 | Edge cases — error display, traits |
| Unit | 2 | Inline |

```bash
cargo test
```

## Ecosystem

| Crate | What |
|-------|------|
| [faf-rust-sdk](https://crates.io/crates/faf-rust-sdk) | Parse, validate, compile .faf files |
| **faf-radio-rust** | Stream context live via Radio Protocol |

Also available in [Bun/TypeScript](https://www.npmjs.com/package/faf-radio) and Zig (coming soon).

## Previously

This crate was published as `mcpaas`. The platform lives on at [mcpaas.live](https://mcpaas.live) — the radio client now has its own name.

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam) | Platform: [MCPaaS](https://mcpaas.live) | Format: [FAF](https://faf.one)
