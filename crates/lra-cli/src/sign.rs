#![forbid(unsafe_code)]

use ed25519_dalek::{Signer, SigningKey};

use crate::util;

/// Maximum key file size (256 bytes — a hex-encoded Ed25519 key is 128 chars).
const MAX_KEY_FILE_SIZE: usize = 256;

/// Maximum receipt file size (1 MB).
const MAX_RECEIPT_SIZE: usize = 1024 * 1024;

/// Maximum hex string length for signature output.
const MAX_SIG_HEX_LEN: usize = 256;

pub fn run(receipt_path: &str, key_path: &str) -> i32 {
    // Step 1: Read the secret key file (bounded)
    let key_file = std::path::Path::new(key_path);
    let key_hex = util::bounded_read_to_string(key_file);
    let key_hex = key_hex.trim();
    if key_hex.is_empty() || key_hex.len() > MAX_KEY_FILE_SIZE {
        eprintln!("Error: Cannot read or invalid key file: {}", key_path);
        return 1;
    }

    // Step 2: Decode hex to 32-byte secret key
    let key_bytes = match hex_decode(key_hex, 64) {
        Some(b) => b,
        None => {
            eprintln!("Error: Invalid hex in key file");
            return 1;
        }
    };
    if key_bytes.len() != 32 {
        eprintln!(
            "Error: Key must be 32 bytes (got {} bytes). Use 'lra keygen' to generate.",
            key_bytes.len()
        );
        return 1;
    }
    let mut key_arr = [0u8; 32];
    let mut ki = 0;
    while ki < 32 {
        key_arr[ki] = key_bytes[ki];
        ki += 1;
    }
    let signing_key = SigningKey::from_bytes(&key_arr);

    // Step 3: Read the receipt JSON file (bounded)
    let receipt_file = std::path::Path::new(receipt_path);
    let receipt_json = util::bounded_read_to_string(receipt_file);
    if receipt_json.is_empty() || receipt_json.len() > MAX_RECEIPT_SIZE {
        eprintln!("Error: Cannot read or oversized receipt: {}", receipt_path);
        return 1;
    }

    // Step 4: Parse as JSON to validate structure
    let mut receipt: serde_json::Value = match serde_json::from_str(&receipt_json) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Invalid JSON in receipt: {}", e);
            return 1;
        }
    };

    // Step 5: Sign the receipt content bytes
    let signature = signing_key.sign(receipt_json.as_bytes());
    let sig_hex = hex_encode(signature.to_bytes().as_ref(), MAX_SIG_HEX_LEN);
    let pub_hex = hex_encode(signing_key.verifying_key().as_bytes(), MAX_SIG_HEX_LEN);

    // Step 6: Add signature fields to JSON
    if let Some(obj) = receipt.as_object_mut() {
        obj.insert("signature".to_string(), serde_json::Value::String(sig_hex));
        obj.insert(
            "signer_pubkey".to_string(),
            serde_json::Value::String(format!("ed25519:{}", pub_hex)),
        );
    } else {
        eprintln!("Error: Receipt must be a JSON object");
        return 1;
    }

    // Step 7: Write signed receipt
    let signed_path = receipt_path.replace(".json", ".signed.json");
    let signed_json = match serde_json::to_string_pretty(&receipt) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Cannot serialize signed receipt: {}", e);
            return 1;
        }
    };
    if let Err(e) = std::fs::write(&signed_path, &signed_json) {
        eprintln!("Error: Cannot write {}: {}", signed_path, e);
        return 1;
    }

    let fingerprint = &pub_hex[..16.min(pub_hex.len())];
    println!("Signed: {} -> {}", receipt_path, signed_path);
    println!("  Signer: ed25519:{}...", fingerprint);
    0
}

/// Hex-decode a string into bytes (bounded by max_bytes).
fn hex_decode(hex: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let bytes_str = hex.as_bytes();
    if !bytes_str.len() % 2 == 0 {
        return None;
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes_str.len() && out.len() < max_bytes {
        let hi = hex_nibble(bytes_str[i])?;
        let lo = hex_nibble(bytes_str[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

/// Convert a hex ASCII byte to its nibble value.
fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Hex-encode a byte slice with bounded output (NASA Power-of-10).
fn hex_encode(bytes: &[u8], max_len: usize) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() && out.len() + 2 <= max_len {
        let b = bytes[i];
        let hi = b >> 4;
        let lo = b & 0x0f;
        out.push(char::from(if hi < 10 { b'0' + hi } else { b'a' + hi - 10 }));
        out.push(char::from(if lo < 10 { b'0' + lo } else { b'a' + lo - 10 }));
        i += 1;
    }
    out
}
