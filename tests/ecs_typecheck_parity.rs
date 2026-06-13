use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_ecs_typecheck_valid_module() {
    let source = r#"
    target profile {
        name: "TestProfile";
        word_size: 64;
    }
    module test_valid {
        signal clk: in bool;
        signal rst: in bool;
        signal a: in u32;
        signal b: in u32;
        signal c: out u32;
        signal cond: internal bool;

        reflex r {
            on always {
                cond = a == b;
                c = a + b;
            }
        }
    }
    "#;

    let config = PipelineConfig {
        typecheck: true,
        // Disable downstream passes to isolate typechecking
        simplify: false,
        width: false,
        temporal: false,
        ..Default::default()
    };

    let result = run_pipeline(source, &config);
    assert!(result.is_ok(), "Valid module failed ECS typechecking: {:?}", result.err());
}

#[test]
fn test_ecs_typecheck_invalid_assignment() {
    let source = r#"
    target profile {
        name: "TestProfile";
        word_size: 64;
    }
    module test_invalid_assignment {
        signal a: in bool;
        signal b: out u32;

        reflex r {
            on always {
                b = a; // Error: Cannot assign bool to u32
            }
        }
    }
    "#;

    let config = PipelineConfig {
        typecheck: true,
        simplify: false,
        width: false,
        temporal: false,
        ..Default::default()
    };

    let result = run_pipeline(source, &config);
    assert!(result.is_err(), "Expected type mismatch error for bool to u32 assignment");
    let errs = result.unwrap_err().errors;
    let msg = format!("{}", errs[0]);
    assert!(msg.contains("is not compatible"), "Unexpected error message: {}", msg);
}

#[test]
fn test_ecs_typecheck_invalid_guard() {
    let source = r#"
    target profile {
        name: "TestProfile";
        word_size: 64;
    }
    module test_invalid_guard {
        signal a: in u32;
        signal b: out u32;

        guard g {
            when a; // Error: Guard condition must be bool
            for 1 cycles;
        }

        reflex r {
            on g {
                b = a;
            }
        }
    }
    "#;

    let config = PipelineConfig {
        typecheck: true,
        simplify: false,
        width: false,
        temporal: false,
        ..Default::default()
    };

    let result = run_pipeline(source, &config);
    assert!(result.is_err(), "Expected type error for non-bool guard condition");
    let errs = result.unwrap_err().errors;
    let msg = format!("{}", errs[0]);
    assert!(msg.contains("condition must be bool"), "Unexpected error message: {}", msg);
}
