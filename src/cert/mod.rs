// ---------------------------------------------------------------------------
//! MEGA-4: Proof certificate format for R-SPU totality verification.
//!
//! Defines the `ProofCertificate` structure, `TerminationStrategy` enum,
//! and binary serialization/deserialization for `.cert` files.
//!
//! Certificate format:
//!   `MIRR_CERT`  8 bytes magic
//!   `version`    1 byte
//!   `body`       variable-length sections
//!
//! Each certificate accompanies a compiled R-SPU binary and proves:
//! - Program hash matches (SHA-256)
//! - Resource bounds fit hardware
//! - Type witnesses for all I/O ports
//! - Property verdicts for all declared properties
//! - Termination strategy + bound
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::totality::ResourceBound;

/// Maximum certificate size (256 KB — more than enough for any R-SPU program).
const MAX_CERT_SIZE: usize = 256 * 1024;

/// Maximum type witnesses in a certificate (one per signal).
const MAX_TYPE_WITNESSES: usize = 4096;

/// Maximum property verdicts in a certificate.
const MAX_PROPERTY_VERDICTS: usize = 4096;

/// Magic bytes identifying a MIRR proof certificate.
const CERT_MAGIC: &[u8; 8] = b"MIRRCERT";

/// Current certificate format version.
const CERT_VERSION: u8 = 1;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// How the termination bound was derived.
/// A bound that explains its own justification is a proof artifact.
/// A bare number is just metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminationStrategy {
    /// All paths are primitive recursive (structurally decreasing).
    PrimitiveRecursive,
    /// Bound derived from maximum static guard cycle count.
    StaticGuardBound { max_guard_cycles: u64 },
    /// Bound derived from hardware resource limits.
    ResourceConstrained { max_instructions: u32, max_registers: u32 },
}

/// Type witness for a signal port (proves type safety at the boundary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeWitness {
    /// Signal name.
    pub name: String,
    /// Signal kind: 0=input, 1=output, 2=internal.
    pub kind: u8,
    /// Bit width.
    pub width: u32,
    /// Signedness: 0=unsigned/bool, 1=signed.
    pub signed: u8,
}

/// Verdict for a declared property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyVerdict {
    /// Property name.
    pub name: String,
    /// Property kind (matches PropertySummary.kind).
    pub kind: String,
    /// Whether the property was structurally verified as total.
    pub verified: bool,
}

/// A proof certificate for an R-SPU binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofCertificate {
    /// Certificate format version.
    pub version: u8,
    /// SHA-256 hash of the program binary (32 bytes).
    pub program_hash: [u8; 32],
    /// Hardware resource bounds.
    pub resource_bound: ResourceBound,
    /// Type witnesses for all I/O signals.
    pub type_witnesses: Vec<TypeWitness>,
    /// Verdicts for all declared properties.
    pub property_verdicts: Vec<PropertyVerdict>,
    /// How the termination bound was derived.
    pub termination_strategy: TerminationStrategy,
    /// Worst-case cycle bound.
    pub termination_bound: u64,
}

// ---------------------------------------------------------------------------
// Serialization — cert → bytes
// ---------------------------------------------------------------------------

/// Serialize a proof certificate to binary format.
///
/// Bounded: output size ≤ MAX_CERT_SIZE.
pub fn serialize_certificate(cert: &ProofCertificate) -> Result<Vec<u8>, String> {
    let mut buf: Vec<u8> = Vec::new();

    // Magic bytes.
    buf.extend_from_slice(CERT_MAGIC);

    // Version.
    buf.push(cert.version);

    // Program hash (32 bytes).
    buf.extend_from_slice(&cert.program_hash);

    // Resource bound (4 fields).
    push_u32(&mut buf, cert.resource_bound.registers);
    push_u32(&mut buf, cert.resource_bound.instructions_estimate);
    push_u32(&mut buf, cert.resource_bound.guards);
    push_u64(&mut buf, cert.resource_bound.max_cycles);

    // Termination strategy tag + data.
    match &cert.termination_strategy {
        TerminationStrategy::PrimitiveRecursive => {
            buf.push(0);
        }
        TerminationStrategy::StaticGuardBound { max_guard_cycles } => {
            buf.push(1);
            push_u64(&mut buf, *max_guard_cycles);
        }
        TerminationStrategy::ResourceConstrained { max_instructions, max_registers } => {
            buf.push(2);
            push_u32(&mut buf, *max_instructions);
            push_u32(&mut buf, *max_registers);
        }
    }

    // Termination bound.
    push_u64(&mut buf, cert.termination_bound);

    // Type witnesses.
    let tw_count = cert.type_witnesses.len().min(MAX_TYPE_WITNESSES);
    push_u32(&mut buf, tw_count as u32);
    let mut twi = 0;
    while twi < tw_count {
        let tw = &cert.type_witnesses[twi];
        push_string(&mut buf, &tw.name);
        buf.push(tw.kind);
        push_u32(&mut buf, tw.width);
        buf.push(tw.signed);
        twi += 1;
    }

    // Property verdicts.
    let pv_count = cert.property_verdicts.len().min(MAX_PROPERTY_VERDICTS);
    push_u32(&mut buf, pv_count as u32);
    let mut pvi = 0;
    while pvi < pv_count {
        let pv = &cert.property_verdicts[pvi];
        push_string(&mut buf, &pv.name);
        push_string(&mut buf, &pv.kind);
        buf.push(if pv.verified { 1 } else { 0 });
        pvi += 1;
    }

    if buf.len() > MAX_CERT_SIZE {
        return Err(format!("Certificate exceeds {} bytes", MAX_CERT_SIZE));
    }

    Ok(buf)
}

// ---------------------------------------------------------------------------
// Deserialization — bytes → cert
// ---------------------------------------------------------------------------

/// Deserialize a proof certificate from binary format.
///
/// Bounded: input size ≤ MAX_CERT_SIZE.
pub fn deserialize_certificate(data: &[u8]) -> Result<ProofCertificate, String> {
    if data.len() > MAX_CERT_SIZE {
        return Err(format!("Certificate exceeds {} bytes", MAX_CERT_SIZE));
    }
    if data.len() < 41 {
        // 8 magic + 1 version + 32 hash = minimum 41
        return Err("Certificate too short".to_string());
    }

    let mut pos: usize = 0;

    // Magic.
    if &data[pos..pos + 8] != CERT_MAGIC {
        return Err("Invalid certificate magic".to_string());
    }
    pos += 8;

    // Version.
    let version = data[pos];
    pos += 1;

    // Program hash.
    let mut program_hash = [0u8; 32];
    program_hash.copy_from_slice(&data[pos..pos + 32]);
    pos += 32;

    // Resource bound.
    let registers = read_u32(data, &mut pos)?;
    let instructions_estimate = read_u32(data, &mut pos)?;
    let guards = read_u32(data, &mut pos)?;
    let max_cycles = read_u64(data, &mut pos)?;

    // Termination strategy.
    if pos >= data.len() {
        return Err("Unexpected end of certificate (strategy)".to_string());
    }
    let strategy_tag = data[pos];
    pos += 1;
    let termination_strategy = match strategy_tag {
        0 => TerminationStrategy::PrimitiveRecursive,
        1 => {
            let max_guard_cycles = read_u64(data, &mut pos)?;
            TerminationStrategy::StaticGuardBound { max_guard_cycles }
        }
        2 => {
            let max_instructions = read_u32(data, &mut pos)?;
            let max_registers = read_u32(data, &mut pos)?;
            TerminationStrategy::ResourceConstrained { max_instructions, max_registers }
        }
        _ => return Err(format!("Unknown termination strategy tag: {}", strategy_tag)),
    };

    // Termination bound.
    let termination_bound = read_u64(data, &mut pos)?;

    // Type witnesses.
    let tw_count = read_u32(data, &mut pos)? as usize;
    if tw_count > MAX_TYPE_WITNESSES {
        return Err(format!("Too many type witnesses: {}", tw_count));
    }
    let mut type_witnesses: Vec<TypeWitness> = Vec::new();
    let mut twi = 0;
    while twi < tw_count {
        let name = read_string(data, &mut pos)?;
        if pos >= data.len() {
            return Err("Unexpected end of certificate (type witness)".to_string());
        }
        let kind = data[pos];
        pos += 1;
        let width = read_u32(data, &mut pos)?;
        if pos >= data.len() {
            return Err("Unexpected end of certificate (type witness signed)".to_string());
        }
        let signed = data[pos];
        pos += 1;
        type_witnesses.push(TypeWitness { name, kind, width, signed });
        twi += 1;
    }

    // Property verdicts.
    let pv_count = read_u32(data, &mut pos)? as usize;
    if pv_count > MAX_PROPERTY_VERDICTS {
        return Err(format!("Too many property verdicts: {}", pv_count));
    }
    let mut property_verdicts: Vec<PropertyVerdict> = Vec::new();
    let mut pvi = 0;
    while pvi < pv_count {
        let name = read_string(data, &mut pos)?;
        let kind = read_string(data, &mut pos)?;
        if pos >= data.len() {
            return Err("Unexpected end of certificate (verdict)".to_string());
        }
        let verified = data[pos] != 0;
        pos += 1;
        property_verdicts.push(PropertyVerdict { name, kind, verified });
        pvi += 1;
    }

    let resource_bound =
        ResourceBound { registers, instructions_estimate, guards, max_cycles, pass: true };

    Ok(ProofCertificate {
        version,
        program_hash,
        resource_bound,
        type_witnesses,
        property_verdicts,
        termination_strategy,
        termination_bound,
    })
}

// ---------------------------------------------------------------------------
// Builder — create certificate from totality result
// ---------------------------------------------------------------------------

/// Build a proof certificate from a totality analysis result and program binary.
pub fn build_certificate(
    totality: &crate::totality::TotalityResult,
    program_binary: &[u32],
    module: &crate::ast::program::Module,
) -> ProofCertificate {
    use crate::ast::types::SignalKind;

    // Compute program hash (SHA-256 of the binary words as little-endian bytes).
    let program_hash = sha256_words(program_binary);

    // Build type witnesses from module signals.
    let mut type_witnesses: Vec<TypeWitness> = Vec::new();
    let mut si = 0;
    while si < module.signals.len() && si < MAX_TYPE_WITNESSES {
        let sig = &module.signals[si];
        let kind = match sig.kind {
            SignalKind::Input => 0,
            SignalKind::Output => 1,
            SignalKind::Internal => 2,
        };
        let (width, is_signed) = sig.ty.core.width_and_signed();
        type_witnesses.push(TypeWitness {
            name: sig.name.clone(),
            kind,
            width,
            signed: if is_signed { 1 } else { 0 },
        });
        si += 1;
    }

    // Build property verdicts from totality summary.
    let mut property_verdicts: Vec<PropertyVerdict> = Vec::new();
    let mut pi = 0;
    while pi < totality.property_summary.len() && pi < MAX_PROPERTY_VERDICTS {
        let ps = &totality.property_summary[pi];
        property_verdicts.push(PropertyVerdict {
            name: ps.name.clone(),
            kind: ps.kind.clone(),
            verified: totality.is_total,
        });
        pi += 1;
    }

    // Determine termination strategy.
    let termination_strategy = if totality.temporal_bound.max_guard_cycles > 0 {
        TerminationStrategy::StaticGuardBound {
            max_guard_cycles: totality.temporal_bound.max_guard_cycles,
        }
    } else {
        TerminationStrategy::PrimitiveRecursive
    };

    ProofCertificate {
        version: CERT_VERSION,
        program_hash,
        resource_bound: totality.resource_bound.clone(),
        type_witnesses,
        property_verdicts,
        termination_strategy,
        termination_bound: totality.temporal_bound.worst_case_latency,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(255) as u8;
    buf.push(len);
    buf.extend_from_slice(&bytes[..len as usize]);
}

fn read_u32(data: &[u8], pos: &mut usize) -> Result<u32, String> {
    if *pos + 4 > data.len() {
        return Err("Unexpected end of certificate (u32)".to_string());
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[*pos..*pos + 4]);
    *pos += 4;
    Ok(u32::from_le_bytes(bytes))
}

fn read_u64(data: &[u8], pos: &mut usize) -> Result<u64, String> {
    if *pos + 8 > data.len() {
        return Err("Unexpected end of certificate (u64)".to_string());
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[*pos..*pos + 8]);
    *pos += 8;
    Ok(u64::from_le_bytes(bytes))
}

fn read_string(data: &[u8], pos: &mut usize) -> Result<String, String> {
    if *pos >= data.len() {
        return Err("Unexpected end of certificate (string len)".to_string());
    }
    let len = data[*pos] as usize;
    *pos += 1;
    if *pos + len > data.len() {
        return Err("Unexpected end of certificate (string data)".to_string());
    }
    let s = String::from_utf8_lossy(&data[*pos..*pos + len]).to_string();
    *pos += len;
    Ok(s)
}

/// Compute SHA-256 of a slice of u32 words (treated as little-endian bytes).
///
/// Uses a minimal implementation (no external crate) — bounded, no heap in
/// the hash core. This is verification-grade, not performance-grade.
fn sha256_words(words: &[u32]) -> [u8; 32] {
    // Convert words to bytes.
    let mut bytes: Vec<u8> = Vec::new();
    let mut i = 0;
    let max = words.len().min(16384); // 64KB max
    while i < max {
        bytes.extend_from_slice(&words[i].to_le_bytes());
        i += 1;
    }
    sha256_bytes(&bytes)
}

/// Minimal SHA-256 implementation (bounded, no unsafe).
///
/// Processes at most 64KB of input (MEGA-4 programs are ≤ 4096 instructions × 4 bytes).
fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // Pre-processing: pad message.
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut padded: Vec<u8> = Vec::new();
    padded.extend_from_slice(data);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) block.
    let mut block_idx = 0;
    let max_blocks = padded.len() / 64;
    while block_idx < max_blocks {
        let offset = block_idx * 64;
        let mut w = [0u32; 64];

        // Load 16 words from the block (big-endian).
        let mut wi = 0;
        while wi < 16 {
            let base = offset + wi * 4;
            w[wi] = ((padded[base] as u32) << 24)
                | ((padded[base + 1] as u32) << 16)
                | ((padded[base + 2] as u32) << 8)
                | (padded[base + 3] as u32);
            wi += 1;
        }

        // Extend to 64 words.
        let mut wi2 = 16;
        while wi2 < 64 {
            let s0 =
                w[wi2 - 15].rotate_right(7) ^ w[wi2 - 15].rotate_right(18) ^ (w[wi2 - 15] >> 3);
            let s1 = w[wi2 - 2].rotate_right(17) ^ w[wi2 - 2].rotate_right(19) ^ (w[wi2 - 2] >> 10);
            w[wi2] = w[wi2 - 16].wrapping_add(s0).wrapping_add(w[wi2 - 7]).wrapping_add(s1);
            wi2 += 1;
        }

        // Compression.
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        let mut ci = 0;
        while ci < 64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 =
                hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[ci]).wrapping_add(w[ci]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
            ci += 1;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);

        block_idx += 1;
    }

    // Produce final hash.
    let mut result = [0u8; 32];
    let mut hi = 0;
    while hi < 8 {
        let bytes = h[hi].to_be_bytes();
        result[hi * 4] = bytes[0];
        result[hi * 4 + 1] = bytes[1];
        result[hi * 4 + 2] = bytes[2];
        result[hi * 4 + 3] = bytes[3];
        hi += 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_roundtrip() {
        let cert = ProofCertificate {
            version: CERT_VERSION,
            program_hash: [0xAA; 32],
            resource_bound: ResourceBound {
                registers: 10,
                instructions_estimate: 42,
                guards: 3,
                max_cycles: 100,
                pass: true,
            },
            type_witnesses: vec![
                TypeWitness { name: "clk".to_string(), kind: 0, width: 1, signed: 0 },
                TypeWitness { name: "out".to_string(), kind: 1, width: 8, signed: 0 },
            ],
            property_verdicts: vec![PropertyVerdict {
                name: "p1".to_string(),
                kind: "always".to_string(),
                verified: true,
            }],
            termination_strategy: TerminationStrategy::StaticGuardBound { max_guard_cycles: 100 },
            termination_bound: 105,
        };

        let bytes = serialize_certificate(&cert).expect("serialize");
        let restored = deserialize_certificate(&bytes).expect("deserialize");

        assert_eq!(restored.version, cert.version);
        assert_eq!(restored.program_hash, cert.program_hash);
        assert_eq!(restored.resource_bound.registers, cert.resource_bound.registers);
        assert_eq!(
            restored.resource_bound.instructions_estimate,
            cert.resource_bound.instructions_estimate
        );
        assert_eq!(restored.resource_bound.guards, cert.resource_bound.guards);
        assert_eq!(restored.resource_bound.max_cycles, cert.resource_bound.max_cycles);
        assert_eq!(restored.type_witnesses.len(), 2);
        assert_eq!(restored.type_witnesses[0].name, "clk");
        assert_eq!(restored.type_witnesses[1].width, 8);
        assert_eq!(restored.property_verdicts.len(), 1);
        assert!(restored.property_verdicts[0].verified);
        assert_eq!(
            restored.termination_strategy,
            TerminationStrategy::StaticGuardBound { max_guard_cycles: 100 }
        );
        assert_eq!(restored.termination_bound, 105);
    }

    #[test]
    fn test_primitive_recursive_strategy_roundtrip() {
        let cert = ProofCertificate {
            version: CERT_VERSION,
            program_hash: [0; 32],
            resource_bound: ResourceBound {
                registers: 1,
                instructions_estimate: 1,
                guards: 0,
                max_cycles: 0,
                pass: true,
            },
            type_witnesses: vec![],
            property_verdicts: vec![],
            termination_strategy: TerminationStrategy::PrimitiveRecursive,
            termination_bound: 0,
        };
        let bytes = serialize_certificate(&cert).expect("serialize");
        let restored = deserialize_certificate(&bytes).expect("deserialize");
        assert_eq!(restored.termination_strategy, TerminationStrategy::PrimitiveRecursive);
    }

    #[test]
    fn test_resource_constrained_strategy_roundtrip() {
        let cert = ProofCertificate {
            version: CERT_VERSION,
            program_hash: [0xFF; 32],
            resource_bound: ResourceBound {
                registers: 256,
                instructions_estimate: 4096,
                guards: 64,
                max_cycles: 999,
                pass: true,
            },
            type_witnesses: vec![],
            property_verdicts: vec![],
            termination_strategy: TerminationStrategy::ResourceConstrained {
                max_instructions: 4096,
                max_registers: 256,
            },
            termination_bound: 999,
        };
        let bytes = serialize_certificate(&cert).expect("serialize");
        let restored = deserialize_certificate(&bytes).expect("deserialize");
        assert_eq!(
            restored.termination_strategy,
            TerminationStrategy::ResourceConstrained { max_instructions: 4096, max_registers: 256 }
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = sha256_bytes(&[]);
        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(hash[0], 0xe3);
        assert_eq!(hash[1], 0xb0);
        assert_eq!(hash[31], 0x55);
    }
}
