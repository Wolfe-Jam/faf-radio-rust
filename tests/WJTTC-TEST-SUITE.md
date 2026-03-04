# WJTTC Test Suite - mcpaas (faf-radio-rust)

**Championship-Grade Testing for Radio Protocol Client (Rust Edition)**

## Test Summary

| Tier | Name | Tests | File | Status |
|------|------|-------|------|--------|
| T1 | BRAKES - Security & Validation | 14 | `tier1_frequency_validation.rs` | 14/14 |
| T2 | ENGINE - Core Functionality | 18 | `tier2_core_functionality.rs` | 18/18 |
| T3 | AERO - Edge Cases & Polish | 12 | `tier3_edge_cases.rs` | 12/12 |
| - | Unit Tests (inline) | 2 | `src/lib.rs` | 2/2 |
| **Total** | | **46** | | |

## Tier 1: BRAKES - 14 Tests

Frequency validation, injection prevention, boundary testing.

| # | Test | What |
|---|------|------|
| 1 | `test_valid_91` | Accept 91.0 FM |
| 2 | `test_valid_boundary_min` | Accept 40.0 FM (min) |
| 3 | `test_valid_boundary_max` | Accept 108.0 FM (max) |
| 4 | `test_invalid_below_min` | Reject 39.9 FM |
| 5 | `test_invalid_above_max` | Reject 108.1 FM |
| 6 | `test_invalid_negative` | Reject -1.0 FM |
| 7 | `test_invalid_non_numeric` | Reject "abc" |
| 8 | `test_injection_sql` | Reject SQL injection |
| 9 | `test_injection_path_traversal` | Reject path traversal |
| 10 | `test_injection_command` | Reject command injection |
| 11 | `test_emoji_frequency` | Reject emoji in frequency |
| 12 | `test_unicode_special_dot` | Reject unicode dot |
| 13 | `test_multiple_valid` | Accept multiple valid freqs |
| 14 | `test_mixed_valid_invalid` | Reject if any invalid |

## Tier 2: ENGINE - 18 Tests

State management, configuration, serialization, WebSocket integration.

| # | Test | What |
|---|------|------|
| 1 | `test_initial_state_disconnected` | New client starts Disconnected |
| 2 | `test_with_url_constructor` | `with_url()` stores URL + defaults |
| 3 | `test_config_default_values` | heartbeat=30s, reconnect=5, delay=1s |
| 4 | `test_config_custom_values` | Non-default config preserved |
| 5 | `test_disconnect_when_disconnected` | No-op, returns Ok |
| 6 | `test_disconnect_sets_state` | State -> Disconnected |
| 7 | `test_connect_already_connected` | AlreadyConnected error (local WS) |
| 8 | `test_tune_when_disconnected` | NotConnected error |
| 9 | `test_untune_when_disconnected` | NotConnected error |
| 10 | `test_tune_invalid_freq_before_send` | InvalidFrequency (not NotConnected) |
| 11 | `test_untune_invalid_freq_before_send` | InvalidFrequency first |
| 12 | `test_tune_empty_vec` | Empty frequencies behavior |
| 13 | `test_tune_action_serialization` | `{"action":"tune","frequencies":[...]}` |
| 14 | `test_untune_action_serialization` | Correct JSON |
| 15 | `test_ping_action_serialization` | `{"action":"ping"}` |
| 16 | `test_server_messages_deserialization` | All 5 ServerMessage variants |
| 17 | `test_connect_to_local_server` | Local WS server, state -> Connected |
| 18 | `test_full_lifecycle` | Connect -> verify -> disconnect -> verify |

## Tier 3: AERO - 12 Tests

Error display, trait behaviors, deserialization edge cases, config boundaries.

| # | Test | What |
|---|------|------|
| 1 | `test_error_display_invalid_frequency` | Contains frequency value |
| 2 | `test_error_display_not_connected` | Readable message |
| 3 | `test_error_display_already_connected` | Readable message |
| 4 | `test_all_error_variants_display` | All 7 testable variants non-empty |
| 5 | `test_connection_state_equality` | Self-equal, cross-unequal |
| 6 | `test_connection_state_clone_copy` | Clone + Copy traits work |
| 7 | `test_connection_state_debug` | Debug output non-empty |
| 8 | `test_unknown_server_message_type` | Deserialization error |
| 9 | `test_missing_required_fields` | Deserialization error |
| 10 | `test_extra_fields_ignored` | Serde default behavior |
| 11 | `test_config_empty_url` | Creates valid config |
| 12 | `test_config_clone` | Independent copy |

## Run

```bash
# All tests
cargo test

# By tier
cargo test tier1
cargo test tier2
cargo test tier3

# With output
cargo test -- --nocapture
```

*WJTTC Certified - Championship Testing Standards*
