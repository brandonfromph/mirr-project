#![forbid(unsafe_code)]

use ed25519_dalek::SigningKey;

/// Maximum keypair output size (hex-encoded, bounded).
const MAX_KEY_HEX_LEN: usize = 128;

pub fn run() -> i32 {
    let mut csprng = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let secret_hex = hex_encode(signing_key.as_bytes(), MAX_KEY_HEX_LEN);
    let public_hex = hex_encode(verifying_key.as_bytes(), MAX_KEY_HEX_LEN);

    let pub_path = std::path::Path::new("lra-identity.pub");
    let key_path = std::path::Path::new("lra-identity.key");

    if let Err(e) = std::fs::write(pub_path, &public_hex) {
        eprintln!("Error writing public key: {}", e);
        return 1;
    }
    if let Err(e) = std::fs::write(key_path, &secret_hex) {
        eprintln!("Error writing secret key: {}", e);
        return 1;
    }

    let fingerprint = &public_hex[..16.min(public_hex.len())];
    println!("Generated Ed25519 keypair:");
    println!("  Public:      {}", pub_path.display());
    println!("  Private:     {}", key_path.display());
    println!("  Fingerprint: ed25519:{}...", fingerprint);
    0
}

/// Hex-encode a byte slice with a bounded output length (NASA Power-of-10).
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
