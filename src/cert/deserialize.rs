//! Binary deserialization of proof certificates.

#![forbid(unsafe_code)]

use super::{
    ProofCertificate, PropertyVerdict, ResourceBound, TerminationStrategy, TypeWitness,
    MAX_CERT_SIZE, MAX_PROPERTY_VERDICTS, MAX_TYPE_WITNESSES,
};

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
    if &data[pos..pos + 8] != super::CERT_MAGIC {
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

pub(super) fn read_u32(data: &[u8], pos: &mut usize) -> Result<u32, String> {
    if *pos + 4 > data.len() {
        return Err("Unexpected end of certificate (u32)".to_string());
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[*pos..*pos + 4]);
    *pos += 4;
    Ok(u32::from_le_bytes(bytes))
}

pub(super) fn read_u64(data: &[u8], pos: &mut usize) -> Result<u64, String> {
    if *pos + 8 > data.len() {
        return Err("Unexpected end of certificate (u64)".to_string());
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[*pos..*pos + 8]);
    *pos += 8;
    Ok(u64::from_le_bytes(bytes))
}

pub(super) fn read_string(data: &[u8], pos: &mut usize) -> Result<String, String> {
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
