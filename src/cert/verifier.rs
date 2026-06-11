//! MEGA-16: Deterministic safety-critical proof verifier for R-SPU.
//!
//! Validates `ProofCertificate` structures against the compiled `RspuProgram`
//! and its binary encoding.

#![forbid(unsafe_code)]

use super::ProofCertificate;
use crate::emit::rspu_isa::{
    RspuProgram, TargetSpec, MAX_GUARDS, MAX_INSTRUCTIONS,
};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// Deterministically verify a `ProofCertificate` against a compiled `RspuProgram` and its binary words.
///
/// Bounded: performs sequential checks on instructions and witnesses.
pub fn verify_certificate(
    cert: &ProofCertificate,
    program: &RspuProgram,
    binary: &[u64],
) -> Result<(), MirrError> {
    let target_spec = TargetSpec::from_config(&program.target);
    let max_registers = target_spec.max_registers();
    let (input_base, output_base, internal_base, temp_base) = target_spec.partitions();

    // 1. Verify program hash (SHA-256 match).
    let computed_hash = super::sha256::sha256_words(binary);
    if computed_hash != cert.program_hash {
        return Err(mirrcode(
            ErrorCode::SignatureVerificationFailed,
            "Certificate program hash does not match R-SPU binary",
        ));
    }

    // 2. Verify physical hardware instruction limits.
    let inst_len = program.instructions.len();
    if inst_len != binary.len() {
        return Err(mirrcode(
            ErrorCode::ReceiptGenerationFailed,
            format!(
                "Instruction length mismatch: program has {}, binary has {}",
                inst_len,
                binary.len()
            ),
        ));
    }
    if inst_len > MAX_INSTRUCTIONS {
        return Err(mirrcode(
            ErrorCode::ReceiptGenerationFailed,
            format!(
                "Instruction count {} exceeds physical hardware limit of {}",
                inst_len, MAX_INSTRUCTIONS
            ),
        ));
    }

    // 3. Verify physical hardware register limits.
    let regs_used = program.registers_used;
    if regs_used > max_registers {
        return Err(mirrcode(
            ErrorCode::ReceiptGenerationFailed,
            format!(
                "Register count {} exceeds physical hardware limit of {}",
                regs_used, max_registers
            ),
        ));
    }

    // 4. Verify physical hardware guard limits.
    let guards_used = program.guards_used;
    if guards_used > MAX_GUARDS {
        return Err(mirrcode(
            ErrorCode::ReceiptGenerationFailed,
            format!(
                "Guard count {} exceeds physical hardware limit of {}",
                guards_used, MAX_GUARDS
            ),
        ));
    }

    // 5. Verify that all property verdicts in the certificate are verified as true.
    let mut pi = 0;
    while pi < cert.property_verdicts.len() {
        let pv = &cert.property_verdicts[pi];
        if !pv.verified {
            return Err(mirrcode(
                ErrorCode::ReceiptGenerationFailed,
                format!("Property '{}' is marked as not verified in the certificate", pv.name),
            ));
        }
        pi += 1;
    }

    // 6. Verify signal type witnesses match the R-SPU program register map partitions.
    let mut twi = 0;
    while twi < cert.type_witnesses.len() {
        let tw = &cert.type_witnesses[twi];
        let mut rmi = 0;
        let mut found_reg = None;
        while rmi < program.register_map.len() {
            let (name, reg_id) = &program.register_map[rmi];
            if name == &tw.name {
                found_reg = Some(*reg_id);
                break;
            }
            rmi += 1;
        }

        if let Some(reg_id) = found_reg {
            match tw.kind {
                0 => {
                    if !(input_base..output_base).contains(&reg_id) {
                        return Err(mirrcode(
                            ErrorCode::ReceiptGenerationFailed,
                            format!(
                                "Input signal '{}' is mapped to register R{}, which is outside the input partition (R{}-R{})",
                                tw.name, reg_id, input_base, output_base - 1
                            ),
                        ));
                    }
                }
                1 => {
                    if !(output_base..internal_base).contains(&reg_id) {
                        return Err(mirrcode(
                            ErrorCode::ReceiptGenerationFailed,
                            format!(
                                "Output signal '{}' is mapped to register R{}, which is outside the output partition (R{}-R{})",
                                tw.name, reg_id, output_base, internal_base - 1
                            ),
                        ));
                    }
                }
                2 => {
                    if !(internal_base..temp_base).contains(&reg_id) {
                        return Err(mirrcode(
                            ErrorCode::ReceiptGenerationFailed,
                            format!(
                                "Internal signal '{}' is mapped to register R{}, which is outside the internal partition (R{}-R{})",
                                tw.name, reg_id, internal_base, temp_base - 1
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(mirrcode(
                        ErrorCode::CertificateSchemaUnsupported,
                        format!("Unknown signal kind {} in witness for '{}'", tw.kind, tw.name),
                    ));
                }
            }
        }
        twi += 1;
    }

    Ok(())
}
