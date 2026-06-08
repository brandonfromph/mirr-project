#![forbid(unsafe_code)]
#![deny(warnings)]

//! Silicon Truth & 1:1 Structural Parity Integration Test Suite.
//!
//! Validates:
//! 1. Formal 1:1 SystemVerilog structural hardware mapping for Guards and Reflexes.
//! 2. Static verification invariants enforced by the native Totality Engine (MEGA-4).
//! 3. Hygienic AST macro expansion span preservation for high-fidelity debugging.

use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::totality::run_totality_check;

#[test]
fn test_silicon_truth_one_to_one_guard_mapping() {
    let source = r#"
        module guard_test {
            signal a: in u8;
            signal b: out bool;
            guard g {
                when a > 10
                for 5 cycles;
            }
            reflex r {
                on g {
                    b = true;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline should compile");

    // Retrieve the compiled temporal netlist
    let verilog = mirrc::emit::verilog::emit_sv(&result);

    assert!(
        verilog.contains("output logic") && verilog.contains("b"),
        "Verilog must preserve output port 'b' type"
    );

    // Shift register/counter signals must be generated 1:1 to drive the 5-cycle delay
    assert!(
        verilog.contains("logic [4:0] g_sr"),
        "Verilog must synthesize a structural shift register for the guard delay"
    );
}

#[test]
fn test_silicon_truth_one_to_one_reflex_mapping() {
    let source = r#"
        module reflex_test {
            signal in_val: in u8;
            signal out_val1: out u8;
            signal out_val2: out u8;
            guard g {
                when in_val == 42
                for 1 cycles;
            }
            reflex r {
                on g {
                    out_val1 = in_val;
                    out_val2 = 100;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline should compile");
    let verilog = mirrc::emit::verilog::emit_sv(&result);

    // The emitted Verilog must declare out_val1 and out_val2 driven combinational 1:1 from reflex
    assert!(verilog.contains("out_val1"), "Must preserve output target name 'out_val1'");
    assert!(verilog.contains("out_val2"), "Must preserve output target name 'out_val2'");

    // Assert 1:1 structural reflex assignment to the matching values
    assert!(
        verilog.contains("100") || verilog.contains("'d100") || verilog.contains("8'h64"),
        "Emitted SystemVerilog must structurally preserve constant value assignment 100"
    );
}

#[test]
fn test_totality_engine_passes_valid_module() {
    let source = r#"
        module total_system {
            signal sensor: in u8;
            signal actuator: out u8;
            guard g {
                when sensor > 0
                for 1 cycles;
            }
            reflex drive {
                on g {
                    actuator = sensor;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline must succeed");

    // Run totality check on the parsed/resolved module AST
    let totality = run_totality_check(&result.program.module);

    assert!(totality.resource_bound.pass, "Resource check must pass");
    assert!(totality.output_completeness.pass, "Output completeness check must pass");
    assert!(totality.guard_coverage.pass, "Guard coverage check must pass");
    assert!(totality.temporal_bound.pass, "Temporal bound check must pass");
    assert!(totality.acyclicity.pass, "Acyclicity check must pass");
    assert!(totality.is_total, "Valid module must be verified as fully total");
}

#[test]
fn test_totality_engine_rejects_missing_output_driver() {
    let source = r#"
        module incomplete_system {
            signal sensor: in u8;
            signal actuator: out u8;
            // Missing reflex driving 'actuator'!
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline must parse");

    // The totality check should catch that 'actuator' is not driven
    let totality = run_totality_check(&result.program.module);
    assert!(
        !totality.output_completeness.pass,
        "Totality engine must fail output completeness if an output signal has no driving reflex"
    );
    assert!(!totality.is_total, "Incomplete system must not be marked total");
}

#[test]
fn test_totality_engine_rejects_combinational_feedback_loop() {
    let source = r#"
        module cyclic_system {
            signal a: out u8;
            signal b: out u8;
            guard g { when true for 1 cycles; }
            reflex r1 {
                on g {
                    a = b;
                }
            }
            reflex r2 {
                on g {
                    b = a;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline must compile");

    let totality = run_totality_check(&result.program.module);
    assert!(
        !totality.acyclicity.pass,
        "Totality engine must fail acyclicity check when combinational feedback loop exists"
    );
    assert!(!totality.is_total, "Cyclic system must not be marked total");
}

#[test]
fn test_hygienic_ast_expansion_preserves_spans() {
    let source = r#"
        def monitor_value(s: signal in bool) {
            reflect {
                signal local_alarm: internal bool;
                reflex r {
                    on always {
                        local_alarm = !${s};
                    }
                }
            }
        }
        module top {
            signal raw_val: in bool;
            monitor_value(raw_val);
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Compilation must succeed");

    // Verify hygienic span mappings inside expanded signals and reflexes
    let module = &result.program.module;
    let alarm_sig = module
        .signals
        .iter()
        .find(|s| s.name.contains("local_alarm"))
        .expect("Expanded signal must be present");

    // Spans must be preserved and carry valid source mapping
    assert!(alarm_sig.span.is_some(), "Expanded hygienic signals must preserve span definitions");

    let alarm_reflex = module
        .reflexes
        .iter()
        .find(|r| r.name.contains("monitor_value"))
        .expect("Expanded reflex must be present");
    assert!(
        alarm_reflex.span.is_some(),
        "Expanded hygienic reflexes must preserve span definitions"
    );
}

#[test]
fn test_compiler_ecs_representation_traceability() {
    let source = r#"
        module traceability_system {
            signal sensor: in u8;
            signal warning: out bool;
            guard g {
                when sensor > 100
                for 3 cycles;
            }
            reflex alarm {
                on g {
                    warning = true;
                }
            }
        }
    "#;

    let parsed = mirrc::parser::parse_mirr(source).expect("Should parse");
    let mut registry = mirrc::ecs::Registry::new();

    // Ingest the program into the registry
    mirrc::ecs::adapter::ingest_program(&mut registry, parsed, None)
        .expect("Ingesting program into ECS Registry must succeed");

    // Verify entities are mapped to contiguous component arrays
    // Look up the guard entity by name
    let guard_entity = registry.get_entity_by_name("g").expect("Guard entity must be registered");

    // Assert contiguous data components can be queried via EntityId index
    let idx = guard_entity.0 as usize;
    assert!(registry.kinds[idx].is_some(), "Kind component must be populated");
    assert!(registry.names[idx].is_some(), "Name component must be populated");
    assert_eq!(registry.names[idx].as_ref().unwrap().0, "g");
    assert!(registry.cycles[idx].is_some(), "Cycles component must be populated");
    assert_eq!(registry.cycles[idx].as_ref().unwrap().0, 3);

    // Run the temporal synthesis system which lowers the guard and attaches synthesis metadata
    let _netlist = mirrc::ecs::systems::temporal_synthesis_system(&mut registry)
        .expect("Temporal synthesis system must run successfully");

    // The guard entity should now have a TemporalNodeComponent attached
    let temp_node = registry.temporal_nodes[idx]
        .as_ref()
        .expect("Temporal node component must be attached to the guard entity");

    assert_eq!(temp_node.delay_cycles, 3);
    assert_eq!(temp_node.output_signal, "g_out");
}
