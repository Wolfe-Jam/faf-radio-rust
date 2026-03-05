# 🏎️ CLAUDE.md - faf-radio-rust Persistent Context & Intelligence

## PROJECT STATE: CHAMPIONSHIP 🥉
**Current Position:** Day 2 Complete - Bronze Tier
**Tyre Compound:** ULTRASOFT C5 (Maximum Performance)
**Test Status:** 46/46 passing (100% - Championship 🏆)
**Deployment:** Published to crates.io (mcpaas@0.1.0)

---

## 🎨 CORE CONTEXT

### Project Identity
- **Name:** faf-radio-rust
- **Version:** 0.1.0
- **Stack:** Rust 2024, Tokio, tokio-tungstenite v0.28
- **Quality:** Championship Performance 🥉
- **Tagline:** "Radio is back. Loved way more than REST."

### Technical Architecture
**What:** Radio Protocol client for Rust - AI Context Broadcasting
**How:** WebSocket-based multiplexed broadcasting (listen-only client)
**Why:** Broadcast once → N AIs receive (99% cost reduction)

**Key Innovation:** Clients tune to frequencies, server broadcasts when souls update

### Architecture Pattern
```
Clients (WebSocket):
  - Connect → Tune → Listen
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
- **Test Coverage:** 46/46 (100%)
- **WJTTC Certification:** All 3 Tiers Complete (T1: 14, T2: 18, T3: 12)
- **Overall Assessment:** Championship Bronze
- **Last Updated:** 2026-02-13

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
- ✅ 46 tests passing
- ✅ 0 tests failing
- ✅ 100% pass rate
- ✅ Zero errors, zero warnings

**Layer 2: WJTTC Championship (3-Tier Coverage)**
- ✅ Tier 1 BRAKES (Security): 14/14 passing
- ✅ Tier 2 ENGINE (Core): 18/18 passing
- ✅ Tier 3 AERO (Edge Cases): 12/12 passing
- ✅ Unit tests: 2/2 passing

**Test Breakdown:**
- Tier 1: Frequency validation (14 tests - Security)
  - Valid ranges, boundary, injection, emoji, unicode
- Tier 2: Core functionality (18 tests - Engine)
  - State management, config, serialization, local WS integration
- Tier 3: Edge cases (12 tests - Aero)
  - Error display, trait behaviors, deserialization, config boundaries
- Unit tests (2 tests - Inline)

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
- `tests/tier2_core_functionality.rs` - Core (18 tests)
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
- [x] WJTTC Tier 2 tests (18/18 passing)
- [x] WJTTC Tier 3 tests (12/12 passing)
- [x] Unit tests (2/2 passing)
- [x] README documentation with 6Ws
- [x] FAF integration (94/100 Bronze 🥉)
- [x] Championship commit

### Next Steps 🔜
- [x] Publish to crates.io ✅ (mcpaas@0.1.0 — 2026-03-04)
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
- **Server → Client:** `{type: 'tuned', frequencies: ['91.0'], ...}`
- Broadcasts are **server-side only** (triggered by soul updates)
- Clients **listen only** (no client-side broadcasting)

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
- Clients tune and listen (WebSocket)
- Server broadcasts when souls update (HTTP POST)
- Separation of concerns: listening ≠ broadcasting
---

**STATUS: BI-SYNC ACTIVE 🔗 - Synchronized with .faf context!**

*Last Sync: 2026-03-05T02:04:22.330Z*
*Sync Engine: F1-Inspired Software Engineering*
*🏎️⚡️_championship_sync*
