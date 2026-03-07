# faf-radio-rust

**Radio Protocol client for Rust.** Broadcast AI context once, every tool receives.

Client SDK for [MCPaaS](https://mcpaas.live) — persistent AI context infrastructure.

## Install

```toml
[dependencies]
faf-radio-rust = "0.2"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

Or via the meta-crate:

```toml
[dependencies]
faf = { version = "0.3", features = ["radio"] }
```

## Quick Start

```rust
use faf_radio_rust::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut radio = RadioClient::new(RadioConfig::grok());

    radio.connect().await?;
    radio.tune(vec!["91.0".to_string()]).await?;

    // Broadcast an event on the frequency
    radio.broadcast("91.0", serde_json::json!({
        "type": "fafb",
        "project": "my-project",
        "size": 220,
    })).await?;

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

## Presets

`RadioConfig::grok()` connects to the FAF beacon with zero configuration:

```rust
// Before (v0.1):
let config = RadioConfig::new("wss://faf-beacon.wolfejam2020.workers.dev/radio");

// After (v0.2):
let config = RadioConfig::grok();
```

## Broadcasting

Clients can now send events on frequencies (v0.2.0):

```rust
// Broadcast FAFb metadata
client.broadcast("91.0", serde_json::json!({
    "type": "fafb",
    "project": "faf-cli",
    "yaml_bytes": 4188,
    "fafb_bytes": 220,
})).await?;
```

## Multi-AI Example

```rust
use faf_radio_rust::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut claude = RadioClient::new(RadioConfig::grok());
    let mut grok   = RadioClient::new(RadioConfig::grok());
    let mut gemini = RadioClient::new(RadioConfig::grok());

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
| `RadioConfig::grok()` | Preset config for Grok inference frequency |
| `RadioConfig::new(url)` | Custom server URL |
| `connect().await` | Connect to beacon |
| `tune(frequencies).await` | Subscribe to frequencies |
| `untune(frequencies).await` | Unsubscribe |
| `broadcast(freq, event).await` | Send event on a frequency |
| `disconnect().await` | Disconnect |
| `state().await` | Get connection state |
| `validate_frequencies(&freqs)` | Validate range (40.0-108.0 FM) |

### Features

- Preset configs (`RadioConfig::grok()`) — zero-config setup
- Client broadcasting — send events on frequencies
- Auto-reconnection with exponential backoff
- Heartbeat (ping/pong every 30s)
- Frequency validation (40.0-108.0 FM)
- Type-safe protocol messages (serde)
- Tokio async/await

## Testing

**50/50 passing** — WJTTC Championship-Grade:

| Tier | Tests | What |
|------|-------|------|
| T1 BRAKES | 14 | Security — injection, boundary, validation |
| T2 ENGINE | 19 | Core — state, config, serialization, broadcast |
| T3 AERO | 12 | Edge cases — error display, traits |
| Unit | 5 | Inline — grok preset, broadcast validation |

```bash
cargo test
```

## Ecosystem

| Crate | What |
|-------|------|
| [faf](https://crates.io/crates/faf) | Meta-crate — `cargo add faf --features radio` |
| [faf-rust-sdk](https://crates.io/crates/faf-rust-sdk) | Parse, validate, compile .faf files |
| **faf-radio-rust** | Stream context live via Radio Protocol |

Also available in [Bun/TypeScript](https://www.npmjs.com/package/faf-radio) and Zig (coming soon).

## Previously

This crate was published as `mcpaas`. The platform lives on at [mcpaas.live](https://mcpaas.live) — the radio client now has its own name.

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam) | Platform: [MCPaaS](https://mcpaas.live) | Format: [FAF](https://faf.one)
