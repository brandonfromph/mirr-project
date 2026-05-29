use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline_on_program, PipelineConfig};

#[test]
fn test_bool_u1_compatibility() {
    let source = "
    module test {
        signal s1: bool;
        signal s2: u1;
        reflex r {
            on always {
                s1 = s2;
                s2 = s1;
            }
        }
    }
    ";
    let program = parse_mirr(source).unwrap();
    let config = PipelineConfig::default();
    let result = run_pipeline_on_program(program, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_width_mismatch_error() {
    let source = "
    module test {
        signal s1: u5;
        signal s2: u32;
        reflex r {
            on always {
                s1 = s2;
            }
        }
    }
    ";
    let program = parse_mirr(source).unwrap();
    let config = PipelineConfig::default();
    let result = run_pipeline_on_program(program, &config);
    assert!(result.is_err());
    let errs = result.expect_err("Expected errors for width mismatch");
    println!("Actual errors: {:?}", errs);
    assert!(errs.errors.iter().any(|e| format!("{:?}", e).contains("E601")));
}

#[test]
fn test_bitwise_mixed_types() {
    let source = "
    module test {
        signal s1: bool;
        signal s2: u32;
        signal s3: u32;
        reflex r {
            on always {
                s3 = s1 && s2;
            }
        }
    }
    ";
    let program = parse_mirr(source).unwrap();
    let config = PipelineConfig::default();
    let result = run_pipeline_on_program(program, &config);
    assert!(
        result.is_err(),
        "Expected type error under standard MIRR for mixed-type logical operands"
    );
    let errs = result.unwrap_err();
    assert!(
        errs.errors.iter().any(|e| format!("{:?}", e).contains("E604")),
        "Expected E604 error, got: {:?}",
        errs
    );
}

#[test]
fn test_index_unsigned() {
    let source = "
    module test {
        signal s1: u5;
        signal b: bool;
        reflex r {
            on always {
                b = s1[0];
            }
        }
    }
    ";
    let program = parse_mirr(source).unwrap();
    let config = PipelineConfig::default();
    let result = run_pipeline_on_program(program, &config);
    if result.is_ok() {
        println!("Indexing unsigned is supported!");
    } else {
        println!("Indexing unsigned failed: {:?}", result.err());
    }
}

#[test]
fn test_index_bool_array() {
    let source = "
    module test {
        signal s1: bool[5];
        signal b: bool;
        reflex r {
            on always {
                b = s1[0];
            }
        }
    }
    ";
    let program = parse_mirr(source).unwrap();
    let config = PipelineConfig::default();
    let result = run_pipeline_on_program(program, &config);
    if result.is_ok() {
        println!("Indexing bool array is supported!");
    } else {
        println!("Indexing bool array failed: {:?}", result.err());
    }
}

#[test]
fn test_bit_assignment() {
    let source = "
    module test {
        signal s1: u5;
        reflex r {
            on always {
                s1[0] = true;
            }
        }
    }
    ";
    let program_result = parse_mirr(source);
    let program = program_result.unwrap();
    let target = &program.module.reflexes[0].assignments[0].target;
    println!("Assignment target: '{}'", target);
}
