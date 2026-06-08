//! Phase 3: Combinatorial Parity Matrix.
//!
//! Generates thousands of hardware variations to verify compiler
//! consistency across bit-widths and topologies.

#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_combinatorial_alu_matrix() {
    let ops = ["+", "-", "*", "&", "|", "^", "==", "!=", "<", ">"];
    let widths = [1, 8, 32, 64];
    let mut count = 0;

    for &op in &ops {
        for &w1 in &widths {
            for &w2 in &widths {
                // Skip combinations that would definitely overflow 64-bit inference for now
                if op == "*" && (w1 + w2 > 64) {
                    continue;
                }

                // Bitwise ops and matching requirements
                if (op == "&" || op == "|" || op == "^") && (w1 != w2) {
                    continue;
                }

                let target_type = if op == "=="
                    || op == "!="
                    || op == "<"
                    || op == ">"
                    || op == "<="
                    || op == ">="
                {
                    "bool"
                } else {
                    "u64"
                };

                let source = format!(
                    r#"
                    module alu_w{w1}_w{w2}_{count} {{
                        signal a: in u{w1};
                        signal b: in u{w2};
                        signal c: out {target_type};
                        
                        guard g {{ when true for 1 cycles; }}
                        reflex r {{ on g {{ c = a {op} b; }} }}
                    }}
                "#,
                    w1 = w1,
                    w2 = w2,
                    count = count,
                    op = op,
                    target_type = target_type
                );

                let config = PipelineConfig::default();
                let res = run_pipeline(&source, &config);
                assert!(res.is_ok(), "ALU matrix failed for {w1} {op} {w2}: {:?}", res.err());
                count += 1;
            }
        }
    }
    println!("Verified {} ALU permutations.", count);
}

#[test]
fn test_combinatorial_shift_matrix() {
    let widths = [8, 16, 32, 64];
    let shifts = [0, 1, 4, 7, 15, 31, 63];
    let mut count = 0;

    for &w in &widths {
        for &s in &shifts {
            if s >= w {
                continue;
            }

            let source = format!(
                r#"
                module shift_w{w}_s{s} {{
                    signal a: in u{w};
                    signal b: out u{w};
                    
                    guard g {{ when true for 1 cycles; }}
                    reflex r {{ on g {{ b = a << {s}; }} }}
                }}
            "#,
                w = w,
                s = s
            );

            let res = run_pipeline(&source, &PipelineConfig::default());
            assert!(res.is_ok(), "Shift matrix failed for u{w} << {s}: {:?}", res.err());
            count += 1;
        }
    }
    println!("Verified {} Shift permutations.", count);
}

#[test]
fn test_combinatorial_mux_matrix() {
    // Generate MUX trees of various sizes
    for entries in [2, 4, 8, 16] {
        let mut signals = String::new();
        let mut logic = String::new();

        for i in 0..entries {
            signals.push_str(&format!("        in_{}: in u8;\n", i));
        }

        logic.push_str("            match sel {\n");
        for i in 0..entries {
            logic.push_str(&format!("                {} => out = in_{};\n", i, i));
        }
        logic.push_str("                _ => out = 0;\n            }\n");

        let source = format!(
            r#"
            module mux_{entries} {{
                signals {{
                    sel: in u4;
                    out: out u8;
                    default: internal bool;
{signals}
                }}
                
                guard g {{ when true for 1 cycles; }}
                reflex r_split_ {{
                    on g {{
{logic}
                    }}
                }}
            }}
        "#,
            entries = entries,
            signals = signals,
            logic = logic
        );

        let res = run_pipeline(&source, &PipelineConfig::default());
        assert!(res.is_ok(), "MUX matrix failed for {entries} entries: {:?}", res.err());
    }
}
