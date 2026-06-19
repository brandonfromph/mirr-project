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

mod deserialize;
mod serialize;
mod sha256;
mod verifier;

pub use deserialize::deserialize_certificate;
pub use serialize::serialize_certificate;
pub use verifier::verify_certificate;

use crate::totality::ResourceBound;

/// Maximum certificate size (256 KB — more than enough for any R-SPU program).
pub(super) const MAX_CERT_SIZE: usize = 256 * 1024;

/// Maximum type witnesses in a certificate (one per signal).
pub(super) const MAX_TYPE_WITNESSES: usize = 4096;

/// Maximum property verdicts in a certificate.
pub(super) const MAX_PROPERTY_VERDICTS: usize = 4096;

/// Magic bytes identifying a MIRR proof certificate.
pub(super) const CERT_MAGIC: &[u8; 8] = b"MIRRCERT";

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
// Builder — create certificate from totality result
// ---------------------------------------------------------------------------

/// Build a proof certificate from a totality analysis result and program binary.
pub fn build_certificate(
    totality: &crate::totality::TotalityResult,
    program_binary: &[u64],
    registry: &crate::ecs::Registry,
) -> ProofCertificate {
    use crate::ast::types::SignalKind;

    // Compute program hash (SHA-256 of the binary words as little-endian bytes).
    let program_hash = sha256::sha256_words(program_binary);

    // Build type witnesses from module signals.
    let mut type_witnesses: Vec<TypeWitness> = Vec::new();
    let mut si = 0;

    for i in 0..registry.names.len() {
        if let (Some(name), Some(kind), Some(ty)) =
            (&registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::components::EntityKind::SIGNAL(skind) = kind.0 {
                let kind_val = match skind {
                    SignalKind::Input => 0,
                    SignalKind::Output => 1,
                    SignalKind::Internal => 2,
                };
                let (width, is_signed) = ty.0.core.width_and_signed();
                type_witnesses.push(TypeWitness {
                    name: registry.resolve_name(name.0).to_string(),
                    kind: kind_val,
                    width,
                    signed: if is_signed { 1 } else { 0 },
                });
                si += 1;
                if si >= MAX_TYPE_WITNESSES {
                    break;
                }
            }
        }
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::sha256::sha256_bytes;
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
