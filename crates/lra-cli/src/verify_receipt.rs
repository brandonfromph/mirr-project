#![forbid(unsafe_code)]

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::util;

/// Maximum receipt file size (1 MB).
const MAX_RECEIPT_SIZE: usize = 1024 * 1024;

/// Maximum key file size (256 bytes).
const MAX_KEY_FILE_SIZE: usize = 256;

pub fn run(receipt_path: &str, pubkey_path: Option<&str>) -> i32 {
    println!("Receipt Verification \u{2014} {}\n", receipt_path);

    // Step 1: Read the signed receipt
    let receipt_file = std::path::Path::new(receipt_path);
    let receipt_json = util::bounded_read_to_string(receipt_file);
    if receipt_json.is_empty() || receipt_json.len() > MAX_RECEIPT_SIZE {
        eprintln!("Error: Cannot read or oversized receipt: {}", receipt_path);
        return 1;
    }

    // Step 2: Parse JSON
    let receipt: serde_json::Value = match serde_json::from_str(&receipt_json) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Invalid JSON: {}", e);
            return 1;
        }
    };

    let obj = match receipt.as_object() {
        Some(o) => o,
        None => {
            eprintln!("Error: Receipt must be a JSON object");
            return 1;
        }
    };

    // Step 3: Extract signature and signer_pubkey
    let sig_hex = match obj.get("signature").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            eprintln!("Error: Receipt has no 'signature' field");
            return 1;
        }
    };

    let embedded_pubkey = obj.get("signer_pubkey").and_then(|v| v.as_str());

    // Step 4: Determine which public key to use
    let pubkey_hex = if let Some(pk_path) = pubkey_path {
        let pk_file = std::path::Path::new(pk_path);
        let hex = util::bounded_read_to_string(pk_file);
        let hex = hex.trim().to_string();
        if hex.is_empty() || hex.len() > MAX_KEY_FILE_SIZE {
            eprintln!("Error: Cannot read or invalid pubkey file: {}", pk_path);
            return 1;
        }
        hex
    } else if let Some(embedded) = embedded_pubkey {
        // Strip "ed25519:" prefix if present
        if let Some(stripped) = embedded.strip_prefix("ed25519:") {
            stripped.to_string()
        } else {
            embedded.to_string()
        }
    } else {
        eprintln!("Error: No public key provided (use --pubkey or embedded signer_pubkey)");
        return 1;
    };

    // Step 5: Decode public key (32 bytes)
    let pubkey_bytes = match hex_decode(&pubkey_hex, 64) {
        Some(b) => b,
        None => {
            eprintln!("Error: Invalid hex in public key");
            return 1;
        }
    };
    if pubkey_bytes.len() != 32 {
        eprintln!("Error: Public key must be 32 bytes (got {})", pubkey_bytes.len());
        return 1;
    }
    let mut pk_arr = [0u8; 32];
    let mut pi = 0;
    while pi < 32 {
        pk_arr[pi] = pubkey_bytes[pi];
        pi += 1;
    }
    let verifying_key = match VerifyingKey::from_bytes(&pk_arr) {
        Ok(vk) => vk,
        Err(e) => {
            eprintln!("Error: Invalid Ed25519 public key: {}", e);
            return 1;
        }
    };

    // Step 6: Decode signature (64 bytes)
    let sig_bytes = match hex_decode(sig_hex, 128) {
        Some(b) => b,
        None => {
            eprintln!("Error: Invalid hex in signature");
            return 1;
        }
    };
    if sig_bytes.len() != 64 {
        eprintln!("Error: Signature must be 64 bytes (got {})", sig_bytes.len());
        return 1;
    }
    let mut sig_arr = [0u8; 64];
    let mut si = 0;
    while si < 64 {
        sig_arr[si] = sig_bytes[si];
        si += 1;
    }
    let signature = Signature::from_bytes(&sig_arr);

    // Step 7: Reconstruct the original content (remove signature + signer_pubkey fields)
    let mut original = obj.clone();
    original.remove("signature");
    original.remove("signer_pubkey");
    let original_json = match serde_json::to_string_pretty(&serde_json::Value::Object(original)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Cannot re-serialize original receipt: {}", e);
            return 1;
        }
    };

    // Step 8: Verify Ed25519 signature
    let valid = verifying_key.verify(original_json.as_bytes(), &signature).is_ok();

    // Step 9: Print results
    let fingerprint = &pubkey_hex[..16.min(pubkey_hex.len())];
    if let Some(target) = obj.get("target_url").and_then(|v| v.as_str()) {
        println!("  Target:    {}", target);
    }
    if let Some(ts) = obj.get("timestamp").and_then(|v| v.as_str()) {
        println!("  Timestamp: {}", ts);
    }
    println!("  Signer:    ed25519:{}...", fingerprint);

    if valid {
        println!("  Signature: VALID");
    } else {
        println!("  Signature: INVALID");
    }

    // Print receipt summary
    if let Some(integrity) = obj.get("integrity").and_then(|v| v.as_str()) {
        println!("\n  Integrity: {}", integrity);
    }
    if let Some(claims) = obj.get("claims_found").and_then(|v| v.as_u64()) {
        println!("  Claims:    {} found", claims);
    }

    println!();
    if valid {
        0
    } else {
        1
    }
}

/// Hex-decode a string into bytes (bounded by max_bytes).
fn hex_decode(hex: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let bytes_str = hex.as_bytes();
    if bytes_str.len() % 2 != 0 {
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
