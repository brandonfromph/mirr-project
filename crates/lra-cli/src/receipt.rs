//! Build certification receipt generation and signing.
//!
//! Implements the receipt format for FDA 21 CFR Part 11 and DO-178C compliance.
//! Receipts are cryptographically signed with Ed25519 and include:
//! - Source file hash (SHA-256)
//! - Compiler version and binary hash
//! - Output artifact hash
//! - Build configuration

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Maximum receipt size in bytes (64 KB).
const MAX_RECEIPT_SIZE: usize = 65536;

/// Schema version for forward compatibility.
const SCHEMA_VERSION: &str = "1.0.0";

/// Build certification receipt for regulatory submission.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReceipt {
    /// Schema version for forward compatibility.
    pub schema_version: String,
    /// Timestamp of receipt generation (ISO 8601).
    pub timestamp: String,
    /// Source file information.
    pub source: SourceInfo,
    /// Compiler information.
    pub compiler: CompilerInfo,
    /// Output artifact information.
    pub output: OutputInfo,
    /// Build configuration.
    pub config: BuildConfig,
    /// Ed25519 signature (hex-encoded, added by signing step).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Signer public key (ed25519:<hex>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_pubkey: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub name: String,
    pub version: String,
    pub binary_hash: String,
    pub git_commit: Option<String>,
    pub rust_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputInfo {
    pub format: String,
    pub path: String,
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub target: String,
    pub opt_level: Option<String>,
    pub flags: Vec<String>,
}

/// Generate an unsigned build receipt.
pub fn generate_receipt(
    source_path: &Path,
    output_path: &Path,
    output_format: &str,
) -> Result<BuildReceipt, String> {
    let source_hash = hash_file(source_path)?;
    let source_meta =
        fs::metadata(source_path).map_err(|e| format!("[E901] Receipt generation failed: {e}"))?;

    let output_hash = hash_file(output_path)?;
    let output_meta =
        fs::metadata(output_path).map_err(|e| format!("[E901] Receipt generation failed: {e}"))?;

    let compiler_binary =
        std::env::current_exe().map_err(|e| format!("[E901] Receipt generation failed: {e}"))?;
    let compiler_hash = hash_file(&compiler_binary)?;

    let now = SystemTime::now();
    let timestamp = format_timestamp(&now);

    let modified = source_meta
        .modified()
        .map(|t| format_timestamp(&t))
        .unwrap_or_else(|_| timestamp.clone());

    Ok(BuildReceipt {
        schema_version: SCHEMA_VERSION.to_string(),
        timestamp,
        source: SourceInfo {
            path: source_path.display().to_string(),
            hash: format!("sha256:{source_hash}"),
            size: source_meta.len(),
            modified,
        },
        compiler: CompilerInfo {
            name: "mirr".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            binary_hash: format!("sha256:{compiler_hash}"),
            git_commit: option_env!("GIT_COMMIT").map(String::from),
            rust_version: option_env!("RUSTC_VERSION").unwrap_or("unknown").to_string(),
        },
        output: OutputInfo {
            format: output_format.to_string(),
            path: output_path.display().to_string(),
            hash: format!("sha256:{output_hash}"),
            size: output_meta.len(),
        },
        config: BuildConfig {
            target: output_format.to_string(),
            opt_level: None,
            flags: Vec::new(),
        },
        signature: None,
        signer_pubkey: None,
    })
}

/// Sign a receipt with Ed25519 key.
pub fn sign_receipt(
    receipt: &mut BuildReceipt,
    secret_key_bytes: &[u8],
) -> Result<(), String> {
    use ed25519_dalek::{Signer, SigningKey};

    // Serialize without signature fields.
    let receipt_json = serde_json::to_string(&receipt)
        .map_err(|e| format!("[E901] Receipt generation failed: {e}"))?;

    if receipt_json.len() > MAX_RECEIPT_SIZE {
        return Err("[E901] Receipt generation failed: receipt too large".to_string());
    }

    // Parse the secret key (expecting 32 bytes for ed25519-dalek).
    if secret_key_bytes.len() < 32 {
        return Err("[E901] Receipt generation failed: invalid key length".to_string());
    }
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&secret_key_bytes[..32]);
    let signing_key = SigningKey::from_bytes(&key_bytes);

    let signature = signing_key.sign(receipt_json.as_bytes());
    let verifying_key = signing_key.verifying_key();

    receipt.signature = Some(hex_encode(&signature.to_bytes()));
    receipt.signer_pubkey = Some(format!("ed25519:{}", hex_encode(verifying_key.as_bytes())));

    Ok(())
}

/// Verify a signed receipt.
pub fn verify_receipt(receipt: &BuildReceipt, pubkey_bytes: &[u8]) -> Result<(), String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let sig_hex = receipt.signature.as_ref().ok_or("[E905] Missing required field: signature")?;
    let sig_bytes = hex_decode(sig_hex).map_err(|_| "[E903] Signature verification failed")?;
    let signature =
        Signature::from_slice(&sig_bytes).map_err(|_| "[E903] Signature verification failed")?;

    if pubkey_bytes.len() < 32 {
        return Err("[E903] Signature verification failed".to_string());
    }
    let mut pk_bytes = [0u8; 32];
    pk_bytes.copy_from_slice(&pubkey_bytes[..32]);
    let verifying_key =
        VerifyingKey::from_bytes(&pk_bytes).map_err(|_| "[E903] Signature verification failed")?;

    // Serialize the receipt without signature fields for verification.
    let unsigned_json = serialize_unsigned(receipt)
        .map_err(|_| "[E903] Signature verification failed")?;

    verifying_key
        .verify(unsigned_json.as_bytes(), &signature)
        .map_err(|_| "[E903] Signature verification failed".to_string())
}

/// Serialize a receipt without signature fields (for signing/verification).
pub fn serialize_unsigned(receipt: &BuildReceipt) -> Result<String, String> {
    // Build a temporary receipt with signature fields cleared.
    let unsigned = BuildReceipt {
        schema_version: receipt.schema_version.clone(),
        timestamp: receipt.timestamp.clone(),
        source: SourceInfo {
            path: receipt.source.path.clone(),
            hash: receipt.source.hash.clone(),
            size: receipt.source.size,
            modified: receipt.source.modified.clone(),
        },
        compiler: CompilerInfo {
            name: receipt.compiler.name.clone(),
            version: receipt.compiler.version.clone(),
            binary_hash: receipt.compiler.binary_hash.clone(),
            git_commit: receipt.compiler.git_commit.clone(),
            rust_version: receipt.compiler.rust_version.clone(),
        },
        output: OutputInfo {
            format: receipt.output.format.clone(),
            path: receipt.output.path.clone(),
            hash: receipt.output.hash.clone(),
            size: receipt.output.size,
        },
        config: BuildConfig {
            target: receipt.config.target.clone(),
            opt_level: receipt.config.opt_level.clone(),
            flags: receipt.config.flags.clone(),
        },
        signature: None,
        signer_pubkey: None,
    };
    serde_json::to_string(&unsigned).map_err(|e| format!("serialization failed: {e}"))
}

/// Format a SystemTime as ISO 8601 string.
fn format_timestamp(time: &SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let year = 1970 + days / 365;
            let month = ((days % 365) / 30) + 1;
            let day = ((days % 365) % 30) + 1;
            format!("{year:04}-{month:02}-{day:02}T00:00:00Z")
        }
        Err(_) => "1970-01-01T00:00:00Z".to_string(),
    }
}

/// Simple hex encode (no external crate).
fn hex_encode(data: &[u8]) -> String {
    let mut result = String::with_capacity(data.len() * 2);
    for &byte in data {
        result.push_str(&format!("{byte:02x}"));
    }
    result
}

/// Simple hex decode (no external crate).
pub fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("invalid hex length".to_string());
    }
    let mut result = Vec::with_capacity(hex.len() / 2);
    let mut i = 0;
    while i < hex.len() {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16).map_err(|_| "invalid hex digit")?;
        result.push(byte);
        i += 2;
    }
    Ok(result)
}

/// Compute SHA-256 hex digest of a file.
fn hash_file(path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};

    let data = fs::read(path).map_err(|e| format!("[E901] Receipt generation failed: {e}"))?;

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(hex_encode(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_file() {
        let path = Path::new("Cargo.toml");
        let hash = hash_file(path).expect("hash should succeed");
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars.
    }
}