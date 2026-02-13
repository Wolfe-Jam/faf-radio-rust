# WJTTC Test Suite - faf-radio-rust

**Championship-Grade Testing for Radio Protocol Client (Rust Edition)**

---

## Test Summary

**Project:** faf-radio-rust
**Type:** Radio Protocol WebSocket Client
**Language:** Rust (2024 edition)
**Framework:** tokio::test + proptest
**Target:** 100% pass rate (Championship 🏆)

---

## Philosophy

> "We break things so others never have to know they were broken."

**Two-Layer Testing:**
- **Layer 1:** Rust standard tests (`cargo test`)
- **Layer 2:** WJTTC expert edge cases

---

## Test Tiers

### Tier 1: BRAKE SYSTEMS 🚨 (Critical Security)
**When failure = catastrophic consequences**

- Frequency validation (injection, overflow, type safety)
- WebSocket security (protocol compliance)
- Memory safety (Rust borrow checker + manual validation)

### Tier 2: ENGINE SYSTEMS ⚡ (Core Functionality)
**When failure = poor experience**

- Connection lifecycle (connect, disconnect, reconnect)
- Protocol messages (tune, untune, broadcasts)
- Heartbeat mechanism

### Tier 3: AERODYNAMICS 🏁 (Polish)
**When failure = minor inconvenience**

- Edge cases (emoji, unicode, special characters)
- Error messages
- State management

---

## Test Plan (42 tests total)

### TIER 1: Security & Validation (12 tests)

**T1.1 - Frequency Validation**
| Test | Expected | Status |
|------|----------|--------|
| Valid: 91.0 FM | Accept | ⏳ |
| Valid: 40.0 FM (min) | Accept | ⏳ |
| Valid: 108.0 FM (max) | Accept | ⏳ |
| Invalid: 39.9 FM | Reject | ⏳ |
| Invalid: 108.1 FM | Reject | ⏳ |
| Invalid: -1.0 FM | Reject | ⏳ |
| Invalid: "abc" | Reject | ⏳ |
| SQL injection: "91.0; DROP TABLE" | Reject | ⏳ |
| Path traversal: "../91.0" | Reject | ⏳ |
| Command injection: "91.0 && rm -rf" | Reject | ⏳ |
| Emoji: "91🏁0" | Reject | ⏳ |
| Unicode: "91․0" (special dot) | Reject | ⏳ |

### TIER 2: Core Functionality (18 tests)

**T2.1 - Connection Lifecycle**
| Test | Expected | Status |
|------|----------|--------|
| Connect to valid server | State = Connected | ⏳ |
| Connect when already connected | Error: AlreadyConnected | ⏳ |
| Disconnect when connected | State = Disconnected | ⏳ |
| Disconnect when not connected | No error | ⏳ |

**T2.2 - Tune/Untune Operations**
| Test | Expected | Status |
|------|----------|--------|
| Tune to 91.0 FM | Success, tuned message | ⏳ |
| Tune to multiple frequencies | Success, all tuned | ⏳ |
| Tune when disconnected | Error: NotConnected | ⏳ |
| Untune from 91.0 FM | Success | ⏳ |
| Tune to invalid frequency | Error before send | ⏳ |

**T2.3 - Message Handling**
| Test | Expected | Status |
|------|----------|--------|
| Receive connected message | Client ID extracted | ⏳ |
| Receive tuned message | Frequencies parsed | ⏳ |
| Receive broadcast message | Event data parsed | ⏳ |
| Receive pong message | Heartbeat logged | ⏳ |
| Receive error message | Error logged | ⏳ |
| Receive malformed JSON | Graceful handling | ⏳ |

**T2.4 - Heartbeat**
| Test | Expected | Status |
|------|----------|--------|
| Ping sent every 30s | Pong received | ⏳ |
| No response to ping | Connection maintained | ⏳ |
| Server initiates ping | Pong sent back | ⏳ |

### TIER 3: Edge Cases & Polish (12 tests)

**T3.1 - Emoji Handling**
| Test | Expected | Status |
|------|----------|--------|
| Emoji in frequency (invalid) | Rejected | ⏳ |
| Emoji in broadcast data | Parsed correctly | ⏳ |
| Emoji in error message | Displayed correctly | ⏳ |
| Mixed emoji + ASCII | Handled correctly | ⏳ |

**T3.2 - Unicode Edge Cases**
| Test | Expected | Status |
|------|----------|--------|
| RTL text in broadcast | Parsed correctly | ⏳ |
| Zero-width characters | Handled | ⏳ |
| Combining characters | Handled | ⏳ |
| Surrogate pairs | Handled | ⏳ |

**T3.3 - Special Characters**
| Test | Expected | Status |
|------|----------|--------|
| Newlines in broadcast | Preserved | ⏳ |
| Quotes in broadcast | Escaped correctly | ⏳ |
| Backslashes | Escaped correctly | ⏳ |
| Null bytes | Handled gracefully | ⏳ |

---

## Execution Plan

**Phase 1: Implement Tier 1 (Security)**
- Critical path: frequency validation
- Target: 12/12 passing

**Phase 2: Implement Tier 2 (Core)**
- Connection lifecycle
- Protocol compliance
- Target: 18/18 passing

**Phase 3: Implement Tier 3 (Edge Cases)**
- Polish and error handling
- Target: 12/12 passing

**Phase 4: Verify Championship**
- All 42 tests passing
- No compiler warnings
- Documentation complete

---

## Championship Scoring

| Pass Rate | Tier | Badge |
|-----------|------|-------|
| 95-100% | Championship | 🏆 |
| 85-94% | Podium | 🥇 |
| 70-84% | Points | 🥈 |
| 55-69% | Midfield | 🥉 |
| <55% | DNF | 🔴 |

---

## Test Commands

```bash
# Run all tests
cargo test

# Run specific tier
cargo test tier1
cargo test tier2
cargo test tier3

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_frequency_validation
```

---

*WJTTC Certified - Championship Testing Standards* 🏎️
