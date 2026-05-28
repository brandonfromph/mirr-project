#![forbid(unsafe_code)]
//! Regression tests for R-SPU guard conjunction and register tag casting.

use nasa_rust_project::emit::rspu_isa::*;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn rspu_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: true,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

/// 1. Verify standard reflexes with zero or single guards do not trigger register conjunction.
#[test]
fn test_reflex_conjunction_zero_guards() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = a;
        }
    }
}
"#;
    let result = run_pipeline(source, &rspu_config()).expect("pipeline should succeed");
    let prog = result.rspu_program.expect("RSPU program not emitted");

    // Zero or single guard should not generate logical ALU AND conjunctions for reflex guards.
    let alu_ands = prog
        .instructions
        .iter()
        .filter(|i| if let RspuInstruction::Alu { op: AluOp::And, .. } = i { true } else { false })
        .count();
    assert_eq!(alu_ands, 0, "No AND logical conjunctions should exist for a single guard");
}

/// 2. Verify nested/compound reflex guard lists correctly conjunctor boolean registers without allocating synthetic hardware guards.
#[test]
fn test_reflex_conjunction_nested_guards() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: in bool;
    signal c: in bool;
    signal d: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    guard g2 {
        when b
        for 1 cycles;
    }

    guard g3 {
        when c
        for 1 cycles;
    }

    reflex r1 {
        on g1 and g2 and g3 {
            d = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &rspu_config()).expect("pipeline should succeed");
    let prog = result.rspu_program.expect("RSPU program not emitted");

    // Conjunction in registers uses ALU AND when there are 3+ guards.
    let has_alu_and =
        prog.instructions.iter().any(|i| matches!(i, RspuInstruction::Alu { op: AluOp::And, .. }));
    assert!(has_alu_and, "Should conjunctor subsequent guards via ALU And in registers");
}

/// 3. Verify that the XOR multiplexer successfully copies and casts the boolean accumulator to the destination's type tag.
#[test]
fn test_tag_casting_multiplexer() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: in bool;
    signal c: out u8;

    guard g1 {
        when a
        for 1 cycles;
    }

    guard g2 {
        when b
        for 1 cycles;
    }

    reflex r1 {
        on g1 and g2 {
            c = 42;
        }
    }
}
"#;
    let result = run_pipeline(source, &rspu_config()).expect("pipeline should succeed");
    let prog = result.rspu_program.expect("RSPU program not emitted");

    // Verify presence of MOV and TAG_LOAD to cast the boolean accumulator to u8 tag (8).
    let has_mov = prog.instructions.iter().any(|i| matches!(i, RspuInstruction::Mov { .. }));
    let has_tag_load_8 = prog.instructions.iter().any(|i| {
        if let RspuInstruction::TagLoad { tag, .. } = i {
            *tag == 8
        } else {
            false
        }
    });

    assert!(has_mov, "Should copy accumulator using MOV");
    assert!(
        has_tag_load_8,
        "Should cast copied accumulator using TAG_LOAD for destination width 8"
    );
}

/// 4. Verify that compiler-level type incompatibility is caught and handled properly before emitter generation.
#[test]
fn test_tag_casting_mismatch_rejection() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out u8;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = a; // Mismatched types: u8 vs bool
        }
    }
}
"#;
    let result = run_pipeline(source, &rspu_config());
    assert!(result.is_err(), "Pipeline should fail with type mismatches before emitter generation");
}

/// 5. Verify that exceeding register allocations (MAX_REGISTERS = 256 / 64 temps) is caught and throws a structured error.
#[test]
fn test_emitter_register_exhaustion_safety() {
    // Generate an extremely nested expression that exhausts temporary registers.
    let mut expr = "a".to_string();
    for _ in 0..100 {
        expr = format!("({} + a)", expr);
    }

    let source = format!(
        r#"
module test_mod {{
    signal a: in u8;
    signal b: out u8;

    guard g1 {{
        when a > 0
        for 1 cycles;
    }}

    reflex r1 {{
        on g1 {{
            b = {};
        }}
    }}
}}
"#,
        expr
    );

    let result = run_pipeline(&source, &rspu_config());
    assert!(result.is_err(), "Should throw error on temporary register exhaustion");
    let err_str = format!("{:?}", result.err().unwrap());
    assert!(err_str.contains("exhausted"), "Should mention register exhaustion, got: {}", err_str);
}
