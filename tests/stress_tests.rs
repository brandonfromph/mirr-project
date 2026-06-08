use mirrc::{ecs::Registry, parse_mirr, run_pipeline, validate_module, PipelineConfig};

fn assert_pass(source: &str) {
    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);
    if let Err(e) = res {
        panic!("Stress test failed to compile: {:?}", e);
    }
}

#[test]
fn stress_s01_nested_prev() {
    // S01: Nested Prev (Boolean combination of prev signals)
    // Testing depth 5 combination.
    let source = "module stress_s01 {
        signal s1: bool;
        signal s2: bool;
        signal s3: bool;
        signal s4: bool;
        signal s5: bool;
        
        guard g1 {
            when prev(s1, 1) && (prev(s2, 2) || (prev(s3, 3) && (prev(s4, 4) || prev(s5, 5)))) for 10
        }
        
        reflex r1 {
            on g1 {
                s1 = false;
            }
        }
    }";
    assert_pass(source);
}

#[test]
fn stress_s02_boolean_tree() {
    // S02: Boolean Tree (10 Ops)
    let source = "module stress_s02 {
        signal s1: bool;
        signal s2: bool;
        signal s3: bool;
        signal s4: bool;
        
        guard g1 {
            when s1 && s2 || s3 && !s4 || (s1 || s2) && (s3 || s4) && !s1 for 1
        }
        
        reflex r1 {
            on g1 {
                s1 = true;
            }
        }
    }";
    assert_pass(source);
}

#[test]
fn stress_s03_symbol_shadowing() {
    // S03: Signal and guard with the same name in the same scope.
    let source = "module s03 { signal x: bool; guard x { when true for 1 cycles; } }";
    let program = parse_mirr(source).expect("should parse");

    // Test AST validator
    let err = validate_module(&program.module).expect_err("should fail AST semantic validation");
    assert!(err.to_string().contains("[E201]"), "expected E201, got: {err}");
    assert!(err.to_string().contains("Name collision"), "should mention name collision");

    // Test ECS validator
    let mut registry = Registry::new();
    registry.ingest_module(&program.module).expect("should ingest");
    let errs = registry.semantic_validate().expect_err("should fail ECS semantic validation");
    let err_msg = format!("{:?}", errs);
    assert!(err_msg.contains("[E201]"), "expected E201 in ECS, got: {err_msg}");
    assert!(err_msg.contains("Name collision"), "should mention name collision in ECS");
}

#[test]
fn stress_s04_massive_signals() {
    // S04: Massive Signal Block (1000 signals)
    let mut source = String::from("module stress_s04 {\n");
    for i in 0..1000 {
        source.push_str(&format!("  signal s{}: u16;\n", i));
    }
    source.push_str("  guard g1 {\n    when s0 == 0 for 1\n  }\n");
    source.push_str("  reflex r1 {\n    on g1 {\n      s1 = 1;\n    }\n  }\n");
    source.push_str("}\n");

    assert_pass(&source);
}
