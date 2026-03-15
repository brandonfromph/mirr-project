// ---------------------------------------------------------------------------
//! MEGA-4: Certificate emission target.
//!
//! Generates a `.cert` proof certificate from a pipeline result with
//! totality checking enabled. The certificate accompanies the R-SPU binary
//! and proves resource bounds, type safety, and termination.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::cert;
use crate::pipeline::PipelineResult;

/// Maximum output file path length (NASA P10 bound).
const MAX_PATH_LEN: usize = 4096;

/// Emit a proof certificate to a file.
///
/// Requires both `totality_result` and `rspu_program` in the pipeline result.
/// Returns the serialized certificate bytes, or an error message.
pub fn emit_certificate(result: &PipelineResult) -> Result<Vec<u8>, String> {
    let totality = result
        .totality_result
        .as_ref()
        .ok_or_else(|| "Totality checking was not enabled (use --totality)".to_string())?;

    if !totality.is_total {
        return Err("Module is not total — cannot generate certificate".to_string());
    }

    let rspu = result
        .rspu_program
        .as_ref()
        .ok_or_else(|| "R-SPU emission required for certificate".to_string())?;

    // Encode R-SPU program to binary for hashing.
    let binary = crate::emit::rspu_encoding::emit_binary(rspu)
        .map_err(|e| format!("R-SPU binary encoding failed: {}", e))?;

    let certificate = cert::build_certificate(totality, &binary, &result.program.module);

    cert::serialize_certificate(&certificate)
}

/// Write a certificate to a `.cert` file alongside the output.
///
/// Returns the number of bytes written, or an error message.
pub fn write_certificate_file(cert_bytes: &[u8], output_path: &str) -> Result<usize, String> {
    if output_path.len() > MAX_PATH_LEN {
        return Err("Output path too long".to_string());
    }

    let cert_path = format!("{}.cert", output_path.trim_end_matches(".rspu"));

    std::fs::write(&cert_path, cert_bytes)
        .map_err(|e| format!("Failed to write certificate: {}", e))?;

    Ok(cert_bytes.len())
}
