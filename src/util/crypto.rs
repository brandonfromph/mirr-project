//! # 🏛️ Sovereign Crypto (Proposal 091)
//! 
//! Centralized cryptographic authority for the Presidential Arsenal.
//! Provides bit-perfect SHA-256 hashing and Ed25519 signing for 
//! LRA receipts and Wave signatures.

use sha2::{Digest, Sha256};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn to_hex_upper(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

/// Calculate the SHA-256 hash of a byte slice.
pub fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let digest = hasher.finalize();
    to_hex_upper(digest.as_ref())
}

/// Generate a new Ed25519 keypair for the Arsenal.
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = VerifyingKey::from(&signing_key);
    (signing_key, verifying_key)
}

/// Sign a message using the provided signing key.
pub fn sign_message(key: &SigningKey, message: &[u8]) -> Signature {
    key.sign(message)
}

/// Verify a signature against a message and verifying key.
pub fn verify_signature(key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    key.verify(message, signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let data = b"MIRR Sovereign Data";
        let h1 = hash_content(data);
        let h2 = hash_content(data);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // SHA-256 hex length
    }

    #[test]
    fn test_signature_roundtrip() {
        let (sk, vk) = generate_keypair();
        let msg = b"Presidential Mandate 091";
        let sig = sign_message(&sk, msg);
        assert!(verify_signature(&vk, msg, &sig));
    }
}
