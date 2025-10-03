//! Utility functions for certificate authority operations
//!
//! This module contains helper functions for formatting distinguished names,
//! serial numbers, and other certificate-related data structures.


#![allow(dead_code)]

use std::collections::HashMap;

/// Format a distinguished name `HashMap` into a string representation
#[must_use]
pub fn format_dn_hashmap<S: ::std::hash::BuildHasher>(dn: &HashMap<String, String, S>) -> String {
    let mut parts = Vec::new();

    // Order DN components in standard order: CN, O, OU, L, ST, C
    let ordered_keys = ["CN", "O", "OU", "L", "ST", "C"];

    for &key in &ordered_keys {
        if let Some(value) = dn.get(key) {
            parts.push(format!("{key}={value}"));
        }
    }

    // Add any remaining keys that weren't in the standard order
    for (key, value) in dn {
        if !ordered_keys.contains(&key.as_str()) {
            parts.push(format!("{key}={value}"));
        }
    }

    if parts.is_empty() {
        "Unknown".to_string()
    } else {
        parts.join(", ")
    }
}

/// Format serial number bytes as hexadecimal string
#[must_use]
pub fn format_serial_number(serial: &[u8]) -> String {
    if serial.is_empty() {
        "00".to_string()
    } else {
        hex::encode(serial)
    }
}
