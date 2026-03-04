# mcpaas

**Your AI forgets you every session. MCPaaS remembers.**

You re-explain your stack to Claude. Switch to Gemini — re-explain everything. Open Cursor — again. Every session. Every tool. Every time.

That's the drift tax. [$49M/day](https://fafdev.tools/value) burned industry-wide.

This SDK connects you to [MCPaaS](https://mcpaas.live) — persistent AI context that follows you across tools, sessions, and teams.

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
mcpaas = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use mcpaas::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut radio = RadioClient::new(
        RadioConfig::new("wss://mcpaas.live/beacon/radio")
    );

    radio.connect().await?;
    radio.tune(vec!["91.0".to_string()]).await?;

    // Your context arrives — from any AI, any session
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    radio.disconnect().await?;
    Ok(())
}
```

That's it. Your context persists. Zero drift.

## How It Works

MCPaaS uses the **Radio Protocol** — broadcast once, every AI receives.

```
Traditional (the tax):
  You → Claude  (send 50KB context)
  You → Grok    (send 50KB again)
  You → Gemini  (send 50KB again)
  = 3x cost, 3x latency, context drift

Radio Protocol (the fix):
  You → Broadcast to 91.0 FM (send once)
  Claude  ← tuned to 91.0
  Grok    ← tuned to 91.0
  Gemini  ← tuned to 91.0
  = 1x cost, instant, zero drift
```

## Multi-AI Example

```rust
use mcpaas::{RadioClient, RadioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://mcpaas.live/beacon/radio";

    let mut claude = RadioClient::new(RadioConfig::new(url));
    let mut grok   = RadioClient::new(RadioConfig::new(url));
    let mut gemini = RadioClient::new(RadioConfig::new(url));

    claude.connect().await?;
    grok.connect().await?;
    gemini.connect().await?;

    claude.tune(vec!["91.0".to_string()]).await?;
    grok.tune(vec!["91.0".to_string()]).await?;
    gemini.tune(vec!["91.0".to_string()]).await?;

    // All three AIs now share the same context, in real time
    Ok(())
}
```

## API

### `RadioClient::new(config: RadioConfig) -> Self`
Create a new Radio client.

### `RadioClient::with_url(url: &str) -> Self`
Create a client with just a URL.

### Methods

| Method | Description |
|--------|-------------|
| `connect().await` | Connect to MCPaaS |
| `tune(frequencies).await` | Subscribe to frequencies |
| `untune(frequencies).await` | Unsubscribe from frequencies |
| `disconnect().await` | Disconnect |
| `state().await` | Get connection state |
| `validate_frequencies(&freqs)` | Validate frequency range (40.0-108.0 FM) |

### Features

- Auto-reconnection with exponential backoff
- Heartbeat mechanism (ping/pong every 30s)
- Frequency validation (40.0-108.0 FM)
- Type-safe protocol messages
- Tokio async/await

## Testing

**46/46 passing** — WJTTC Championship-Grade 3-Tier coverage:

| Tier | Tests | What |
|------|-------|------|
| T1 BRAKES | 14 | Security — injection, boundary, validation |
| T2 ENGINE | 18 | Core — state, config, serialization, WebSocket |
| T3 AERO | 12 | Polish — error display, traits, edge cases |
| Unit | 2 | Inline |

```bash
cargo test
```

## See Also

- **[faf-rust-sdk](https://crates.io/crates/faf-rust-sdk)** — Parse, validate, and compress .faf files in Rust. MCPaaS broadcasts the context; faf-rust-sdk reads the format.

**Do I need both?** Yes. `faf-rust-sdk` parses your .faf project DNA. `mcpaas` streams it live to every AI. One reads, the other delivers.

## Namepoints

Every frequency maps to a **namepoint** on MCPaaS — your permanent AI identity.

Free namepoints work. But `yourname.mcpaas.live` hits different than `user-38291.mcpaas.live`.

**Claim yours before someone else does:** [mcpaas.live/claim](https://mcpaas.live/claim)

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam) | Powered by [MCPaaS](https://mcpaas.live) | Format: [FAF](https://faf.one)
