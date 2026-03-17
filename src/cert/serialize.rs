//! Binary serialization of proof certificates.

#![forbid(unsafe_code)]

use super::{
    ProofCertificate, TerminationStrategy, MAX_CERT_SIZE, MAX_PROPERTY_VERDICTS, MAX_TYPE_WITNESSES,
};

/// Serialize a proof certificate to binary format.
///
/// Bounded: output size ≤ MAX_CERT_SIZE.
pub fn serialize_certificate(cert: &ProofCertificate) -> Result<Vec<u8>, String> {
    let mut buf: Vec<u8> = Vec::new();

    // Magic bytes.
    buf.extend_from_slice(super::CERT_MAGIC);

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

pub(super) fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub(super) fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub(super) fn push_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(255) as u8;
    buf.push(len);
    buf.extend_from_slice(&bytes[..len as usize]);
}
