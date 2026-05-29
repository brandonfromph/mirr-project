//! TDD Integration Tests: Proof-Carrying Code (PCC) Verification.
//!
//! Verifies the core safety-critical proof verifier `verifier.rs` against
//! compiled R-SPU modules and checks physical hardware boundaries and round-trips.

#![forbid(unsafe_code)]

use nasa_rust_project::cert::{deserialize_certificate, verify_certificate};
use nasa_rust_project::emit::rspu_encoding::emit_binary;
use nasa_rust_project::emit::rspu_isa::{
    RspuInstruction, MAX_GUARDS, MAX_INSTRUCTIONS, MAX_REGISTERS,
};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const FIXTURE_SAFE_COUNTER: &str = r#"
module safe_counter {
    signals {
        clk: in bool
        rst: in bool
        out_val: out u8
    }

    guard g_tick {
        when rst == false
        for 1 cycles;
    }

    reflex increment {
        on g_tick {
            out_val = out_val + 1;
        }
    }
}
"#;

#[test]
fn test_pcc_verification_safe_module_passes() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");
    let binary = emit_binary(rspu).expect("emit binary");

    // Deterministic verify must pass!
    let verify_result = verify_certificate(&cert, rspu, &binary);
    assert!(
        verify_result.is_ok(),
        "Safe module certificate verification failed: {:?}",
        verify_result
    );
}

#[test]
fn test_pcc_verification_fails_on_hash_tamper() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let mut cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");
    let binary = emit_binary(rspu).expect("emit binary");

    // Tamper with the certificate program hash
    cert.program_hash[0] ^= 0xFF;

    let verify_result = verify_certificate(&cert, rspu, &binary);
    assert!(verify_result.is_err(), "Tampered hash verification should fail");
}

#[test]
fn test_pcc_verification_fails_on_binary_tamper() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");
    let mut binary = emit_binary(rspu).expect("emit binary");

    // Tamper with the compiled binary
    if !binary.is_empty() {
        binary[0] ^= 0xFFFFFFFF;
    }

    let verify_result = verify_certificate(&cert, rspu, &binary);
    assert!(verify_result.is_err(), "Tampered binary verification should fail");
}

#[test]
fn test_pcc_verification_fails_on_registers_exceeded() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");
    let binary = emit_binary(rspu).expect("emit binary");

    // Simulate exceeding the physical hardware register limit
    let mut bad_program = rspu.clone();
    bad_program.registers_used = MAX_REGISTERS + 1;

    let verify_result = verify_certificate(&cert, &bad_program, &binary);
    assert!(verify_result.is_err(), "Exceeded physical registers should fail");
}

#[test]
fn test_pcc_verification_fails_on_guards_exceeded() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");
    let binary = emit_binary(rspu).expect("emit binary");

    // Simulate exceeding the physical hardware guard limit
    let mut bad_program = rspu.clone();
    bad_program.guards_used = MAX_GUARDS + 1;

    let verify_result = verify_certificate(&cert, &bad_program, &binary);
    assert!(verify_result.is_err(), "Exceeded physical guards should fail");
}

#[test]
fn test_pcc_verification_fails_on_instructions_exceeded() {
    let config = PipelineConfig { rspu: true, totality: true, ..Default::default() };

    let result = run_pipeline(FIXTURE_SAFE_COUNTER, &config).expect("pipeline compile");

    let rspu = result.rspu_program.as_ref().expect("rspu program missing");
    let cert_bytes = rspu.certificate.as_ref().expect("certificate missing");

    let cert = deserialize_certificate(cert_bytes).expect("deserialize certificate");

    // Simulate exceeding physical instruction capacity limit by adding instructions
    let mut bad_program = rspu.clone();
    let mut bad_binary = emit_binary(rspu).expect("emit binary");

    let mut i = bad_program.instructions.len();
    while i <= MAX_INSTRUCTIONS {
        bad_program.instructions.push(RspuInstruction::Nop);
        bad_binary.push(0); // Nop opcode representation
        i += 1;
    }

    let verify_result = verify_certificate(&cert, &bad_program, &bad_binary);
    assert!(verify_result.is_err(), "Exceeded instructions bound should fail");
}
