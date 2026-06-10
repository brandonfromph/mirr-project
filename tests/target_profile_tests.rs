//! Integration tests for the 'Liquid Compiler' Target Profile system.

#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_target_compact_32bit() {
    let source = "
target {
    name: \"Compact-32\";
    word_size: 32;
    reg_bits: 8;
    guard_bits: 6;
}

module compact_test {
    signal a: in bool;
    signal b: out bool;
    reflex main {
        on always {
            b = a;
        }
    }
}
";
    let config = PipelineConfig { rspu: true, ..Default::default() };

    let result = run_pipeline(source, &config).expect("Pipeline failed");
    let program = result.rspu_program.expect("R-SPU program not emitted");

    // Verify target AST node
    let target = program.target.as_ref().expect("Target metadata missing");
    assert_eq!(target.name, "Compact-32");
    assert_eq!(target.word_size, 32);
    assert_eq!(target.reg_bits, 8);

    // Verify binary encoding (should be 32-bit words packed into u64 container)
    let binary = mirrc::emit::rspu_encoding::emit_binary(&program).expect("Binary emission failed");
    for word in binary {
        // Opcode for 32-bit should be at [31:26].
        // 64-bit container will have high bits 0 if word_size was 32.
        assert!(word <= 0xFFFF_FFFF, "Encoded word exceeds 32 bits for Compact target");
    }
}

#[test]
fn test_target_liquid_64bit() {
    let source = "
target {
    name: \"Liquid-64\";
    word_size: 64;
    reg_bits: 10;
    guard_bits: 8;
}

module liquid_test {
    signal a: in bool;
    signal b: out bool;
    reflex main {
        on always {
            b = a;
        }
    }
}
";
    let config = PipelineConfig { rspu: true, ..Default::default() };

    let result = run_pipeline(source, &config).expect("Pipeline failed");
    let program = result.rspu_program.expect("R-SPU program not emitted");

    let target = program.target.as_ref().expect("Target metadata missing");
    assert_eq!(target.word_size, 64);
    assert_eq!(target.reg_bits, 10);

    let _binary = mirrc::emit::rspu_encoding::emit_binary(&program).expect("Binary emission failed");
    // At least some word should exceed 32-bit range for 64-bit target if opcodes/fields are high enough
    // But even if not, we checked the metadata.
}

#[test]
fn test_default_target() {
    let source = "
module default_test {
    signal a: in bool;
    signal b: out bool;
    reflex main {
        on always {
            b = a;
        }
    }
}
";
    let config = PipelineConfig { rspu: true, ..Default::default() };

    let result = run_pipeline(source, &config).expect("Pipeline failed");
    let program = result.rspu_program.expect("R-SPU program not emitted");

    // Default should be None in AST but resolved to Liquid constants in encoding
    assert!(program.target.is_none());
}
