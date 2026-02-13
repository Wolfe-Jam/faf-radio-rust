use faf_radio_rust::{RadioClient, RadioConfig};

/// TIER 1: BRAKE SYSTEMS - Frequency Validation (Security Critical)
/// These tests ensure the client rejects malicious or invalid frequency inputs

#[test]
fn test_valid_frequency_91_0() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["91.0".to_string()]);
    assert!(result.is_ok(), "91.0 FM should be valid");
}

#[test]
fn test_valid_frequency_min_40_0() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["40.0".to_string()]);
    assert!(result.is_ok(), "40.0 FM (minimum) should be valid");
}

#[test]
fn test_valid_frequency_max_108_0() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["108.0".to_string()]);
    assert!(result.is_ok(), "108.0 FM (maximum) should be valid");
}

#[test]
fn test_invalid_frequency_below_min() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["39.9".to_string()]);
    assert!(result.is_err(), "39.9 FM should be rejected (below minimum)");
}

#[test]
fn test_invalid_frequency_above_max() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["108.1".to_string()]);
    assert!(result.is_err(), "108.1 FM should be rejected (above maximum)");
}

#[test]
fn test_invalid_frequency_negative() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["-1.0".to_string()]);
    assert!(result.is_err(), "-1.0 FM should be rejected (negative)");
}

#[test]
fn test_invalid_frequency_non_numeric() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["abc".to_string()]);
    assert!(result.is_err(), "Non-numeric frequency should be rejected");
}

#[test]
fn test_sql_injection_attempt() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["91.0; DROP TABLE users".to_string()]);
    assert!(result.is_err(), "SQL injection attempt should be rejected");
}

#[test]
fn test_path_traversal_attempt() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["../91.0".to_string()]);
    assert!(result.is_err(), "Path traversal attempt should be rejected");
}

#[test]
fn test_command_injection_attempt() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["91.0 && rm -rf /".to_string()]);
    assert!(result.is_err(), "Command injection attempt should be rejected");
}

#[test]
fn test_emoji_in_frequency() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&["91🏁0".to_string()]);
    assert!(result.is_err(), "Emoji in frequency should be rejected");
}

#[test]
fn test_unicode_special_dot() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    // Using Unicode dot (․) instead of ASCII period (.)
    let result = client.validate_frequencies(&["91․0".to_string()]);
    assert!(result.is_err(), "Unicode special dot should be rejected");
}

#[test]
fn test_multiple_valid_frequencies() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&[
        "91.0".to_string(),
        "92.5".to_string(),
        "94.7".to_string(),
    ]);
    assert!(result.is_ok(), "Multiple valid frequencies should be accepted");
}

#[test]
fn test_mixed_valid_invalid_frequencies() {
    let config = RadioConfig::new("wss://example.com");
    let client = RadioClient::new(config);

    let result = client.validate_frequencies(&[
        "91.0".to_string(),
        "999.9".to_string(), // Invalid
    ]);
    assert!(result.is_err(), "Mixed valid/invalid should be rejected");
}
