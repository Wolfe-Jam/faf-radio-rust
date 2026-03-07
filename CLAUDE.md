# 🏎️ CLAUDE.md - faf-radio-rust Persistent Context & Intelligence

## PROJECT STATE: CHAMPIONSHIP 🥉
**Current Position:** v0.2.0 — Grok Presets + Client Broadcast
**Tyre Compound:** ULTRASOFT C5 (Maximum Performance)
**Test Status:** 50/50 passing (100% - Championship 🏆)
**Deployment:** Published to crates.io (faf-radio-rust@0.2.0)

---

## 🎨 CORE CONTEXT

### Project Identity
- **Name:** faf-radio-rust
- **Version:** 0.2.0
- **Stack:** Rust 2024, Tokio, tokio-tungstenite v0.28
- **Quality:** Championship Performance 🥉
- **Tagline:** "Radio is back. Loved way more than REST."

### Technical Architecture
**What:** Radio Protocol client for Rust - AI Context Broadcasting
**How:** WebSocket-based multiplexed broadcasting (tune, listen, and broadcast)
**Why:** Broadcast once → N AIs receive (99% cost reduction)

**Key Innovation:** Clients tune to frequencies, server broadcasts when souls update

### Architecture Pattern
```
Clients (WebSocket):
  - Connect → Tune → Listen / Broadcast
  - RadioConfig::grok() preset (zero-config)
  - client.broadcast(freq, event) — send events on frequencies
  - Auto-reconnect with exponential backoff
  - Heartbeat every 30s

Server (HTTP + WebSocket):
  - POST /nelly/write → Broadcast to 91.0 FM
  - All tuned clients receive instantly
```

### Available Frequencies
- **91.0 FM** - nelly (personal context)
- **92.5 FM** - faf (project updates)
- **94.7 FM** - wolfejam (team context)

### 📊 Context Quality Status
- **FAF Score:** 94/100 🥉 Bronze (genuine faf-cli workflow)
- **Birth DNA:** 29% → **Growth:** +65% in 21 minutes
- **Test Coverage:** 50/50 (100%)
- **WJTTC Certification:** All 3 Tiers Complete (T1: 14, T2: 19, T3: 12) + 5 unit
- **Overall Assessment:** Championship Bronze
- **Last Updated:** 2026-03-06

---

## 🔗 ECOSYSTEM POSITION

**Part of Triple Launch Strategy:**
- **Day 1:** Bun Radio Client ✅ COMPLETE (100/100 Trophy)
- **Day 2:** Rust FAFb Client ✅ COMPLETE (this repo - 94/100 Bronze)
- **Day 3:** Zig WASM (2.7KB) 🔜 NEXT

**Server Infrastructure:**
- mcpaas-beacon: Radio Protocol server (103/103 tests passing)
- Deployed: Cloudflare Workers (300+ edge locations)
- Protocol: FAF Radio Protocol v1.0

---

## 🧪 TESTING

### Test Results (Championship 🏆)

**Layer 1: Industry Standard (Rust cargo test)**
- ✅ 50 tests passing
- ✅ 0 tests failing
- ✅ 100% pass rate
- ✅ Zero errors, zero warnings, clippy clean

**Layer 2: WJTTC Championship (3-Tier Coverage)**
- ✅ Tier 1 BRAKES (Security): 14/14 passing
- ✅ Tier 2 ENGINE (Core): 19/19 passing
- ✅ Tier 3 AERO (Edge Cases): 12/12 passing
- ✅ Unit tests: 5/5 passing

**Test Breakdown:**
- Tier 1: Frequency validation (14 tests - Security)
  - Valid ranges, boundary, injection, emoji, unicode
- Tier 2: Core functionality (19 tests - Engine)
  - State management, config, serialization, broadcast serialization, local WS integration
- Tier 3: Edge cases (12 tests - Aero)
  - Error display, trait behaviors, deserialization, config boundaries
- Unit tests (5 tests - Inline)
  - Frequency validation, initial state, grok preset, broadcast invalid freq, broadcast when disconnected

**Run tests:** `cargo test`

---

## 📂 KEY FILES

### Source Code
- `src/lib.rs` - Main Radio client (~230 lines)
- `src/types.rs` - Rust types for protocol
- `src/error.rs` - Error types (thiserror)

### Examples
- `examples/basic.rs` - Connect, tune, listen
- `examples/multi_ai.rs` - 3 AIs on one frequency

### Tests
- `tests/WJTTC-TEST-SUITE.md` - Full test plan
- `tests/tier1_frequency_validation.rs` - Security (14 tests)
- `tests/tier2_core_functionality.rs` - Core (19 tests)
- `tests/tier3_edge_cases.rs` - Edge cases (12 tests)

### Documentation
- `README.md` - API reference, examples, 6Ws
- `CLAUDE.md` - This file (persistent context)
- `project.faf` - AI-readable context (94/100 Bronze)

---

## 🚀 CURRENT STATUS

### Completed ✅
- [x] Rust Radio client implementation
- [x] Protocol alignment with server (action/frequencies)
- [x] WebSocket connection management
- [x] Auto-reconnection with exponential backoff
- [x] Heartbeat mechanism (ping/pong)
- [x] Event-driven API
- [x] Frequency validation (40.0-108.0 FM)
- [x] WJTTC Tier 1 tests (14/14 passing)
- [x] WJTTC Tier 2 tests (19/19 passing)
- [x] WJTTC Tier 3 tests (12/12 passing)
- [x] Unit tests (5/5 passing)
- [x] README documentation with 6Ws
- [x] FAF integration (94/100 Bronze 🥉)
- [x] Championship commit
- [x] Publish v0.1.0 to crates.io (2026-03-04)
- [x] RadioConfig::grok() preset (v0.2.0)
- [x] ClientAction::Broadcast + RadioClient::broadcast() (v0.2.0)
- [x] Publish v0.2.0 to crates.io (2026-03-06)

### Next Steps 🔜
- [ ] Add CI/CD (GitHub Actions)
- [ ] Add more examples
- [ ] Documentation improvements

---

## 🔧 DEVELOPMENT

### Quick Start
```bash
# Run examples
cargo run --example basic
cargo run --example multi_ai

# Run tests
cargo test

# Check FAF score
faf score
```

### Protocol Notes
- **Client → Server:** `{action: 'tune', frequencies: ['91.0']}`
- **Client → Server:** `{action: 'broadcast', frequency: '91.0', event: {...}}`
- **Server → Client:** `{type: 'tuned', frequencies: ['91.0'], ...}`
- **Server → Client:** `{type: 'broadcast', frequency: '91.0', event: {...}, timestamp: '...'}`
- Clients can now both listen AND broadcast (v0.2.0)
- `RadioConfig::grok()` — zero-config preset for Grok inference frequency

### Server URLs
- **Production:** `wss://faf-beacon.wolfejam2020.workers.dev/radio`
- **API Info:** `https://mcpaas.live/beacon`

---

## 💡 KEY INSIGHTS

### FAF Workflow Demonstrated
**Day 2 Achievement:** Demonstrated genuine FAF CLI workflow

**Journey:**
1. `faf init` → 29% birth DNA (real tool output)
2. `faf auto` → Tool auto-detection
3. `faf go` → Guided interview (manual input)
4. Manual edits → Proper `slotignore` syntax
5. Final score → 94/100 Bronze 🥉 (compiler verified)

**Not AI-crafted YAML** - This is authentic FAF tool behavior!

### Technical Highlights
- Rust 2024 edition (stabilized Feb 2025)
- tokio-tungstenite v0.28 (production-ready)
- `tokio::select!` for concurrent message handling
- `Arc<RwLock<T>>` for thread-safe state
- thiserror for ergonomic error handling

### Architecture Clarification
- Clients tune, listen, and broadcast (WebSocket) — full protocol symmetry since v0.2.0
- Server also broadcasts when souls update (HTTP POST)
- `RadioConfig::grok()` replaces manual `RadioConfig::new("wss://faf-beacon...")`
---

**STATUS: BI-SYNC ACTIVE 🔗 - Synchronized with .faf context!**

*Last Sync: 2026-03-06T07:51:06.198Z*
*Sync Engine: F1-Inspired Software Engineering*
*🏎️⚡️_championship_sync*
