use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_ecs_simplify_parity() {
    let source = r#"
    target profile {
        name: "SimplifyTest";
        word_size: 32;
        reg_width: 8;
        op_width: 8;
    }

    module test_simplifier {
        signal a: in bool;
        signal b: in bool;
        signal out1: out bool;
        signal out2: out bool;

        reflex r {
            on always {
                // Should reduce to: out1 = a
                out1 = (a && true) || false;
            }
        }
        
        reflex r2 {
            on always {
                // Should reduce to false
                out2 = b && !b;
            }
        }
    }
    "#;

    let config = PipelineConfig {
        simplify: true,
        temporal: false, // Disable downstream
        width: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(source, &config).expect("Pipeline should run");

    // Extract the ECS simplify stats
    let stats = result.simplify_stats.expect("Simplify stats should be present");

    // We expect rules to have fired (true folded, false folded)
    assert!(stats.rules_applied > 0, "ECS simplifier should apply rules");
}
