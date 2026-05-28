#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

/// CONTRACT: The MIRR compiler should support "Rust-like" syntactic sugar
/// for its core primitives (Signal, Guard, Reflex) to improve ergonomics
/// in large chip designs.
#[test]
fn test_rust_like_syntax_sugar() {
    let source = r#"
module advanced_controller {
    // 1. Ergonimc signal block (Rust-like 'let' or 'pub' style)
    signals {
        clk: in bool;
        rst: in bool;
        data_out: out u16;
    }

    // 2. Ergonimc guard syntax
    guard ready = when clk for 10 cycles;

    // 3. Rust-like reflex syntax (if/else instead of on/when)
    reflex controller_logic {
        if ready {
            data_out = 1;
        } else if rst {
            data_out = 0;
        }
    }
}
"#;

    // The macro processor should be responsible for lowering this sugar
    let expanded = expand_macros(source);
    println!("--- EXPANDED SOURCE ---\n{}\n--- END EXPANDED ---", expanded);

    // Verify that the expansion produces standard MIRR syntax that the parser accepts.
    // Standard reflex syntax uses:
    // reflex <name> {
    //    on <guard> { <assignments> }
    // }

    let program =
        parse_mirr(&expanded).expect("Parser should accept Rust-like sugar after expansion");

    assert_eq!(program.module.name, "advanced_controller");
    assert_eq!(program.module.signals.len(), 3);
    assert_eq!(program.module.guards.len(), 2);

    // The if/else-if logic should be lowered to separate reflexes or combined with OR logic if possible.
    // For now, we expect two separate reflexes or a single reflex with multiple trigger guards.
    assert!(!program.module.reflexes.is_empty());
}

#[test]
fn test_sugar_assignment_shorthand() {
    let source = r#"
module shortcut {
    signals {
        a: in bool;
        b: out bool;
    }
    
    // Shorthand for simple reflex: a -> b
    reflex pass { b = a; } 
}
"#;

    let expanded = expand_macros(source);
    println!("--- SHORTCUT EXPANDED SOURCE ---\n{}\n--- END SHORTCUT ---", expanded);
    let _program = parse_mirr(&expanded).expect("Shorthand assignment should work");
}
#[test]
fn test_property_after_reflex() {
    let source = r#"
module debug_mod {
    signals {
        a: in bool;
        b: out bool;
    }
    
    reflex r1 {
        on a {
            b = true;
        }
    }
    
    property p1 {
        always (a -> b);
    }
}
"#;
    let expanded = expand_macros(source);
    println!("--- PROPERTY AFTER REFLEX EXPANDED ---\n{}\n--- END ---", expanded);
    let _program = parse_mirr(&expanded).expect("Property after reflex should work");
}

#[test]
fn test_let_binding_expansion() {
    let source = r#"
module let_binding_test {
    signals {
        clk: in bool;
        data_out: out u16;
    }
    
    reflex r1 {
        if clk {
            let tmp: u16 = 42;
            data_out = tmp;
        }
    }
}
"#;
    let expanded = expand_macros(source);
    println!("--- LET BINDING EXPANDED ---\n{}\n--- END ---", expanded);

    assert!(expanded.contains("signal tmp: internal u16;"));
    assert!(expanded.contains("tmp = 42;"));

    let program = parse_mirr(&expanded).expect("Let binding expansion should parse successfully");
    assert_eq!(program.module.signals.len(), 3);
}

#[test]
fn test_match_expression_expansion() {
    let source = r#"
module match_test {
    signals {
        state: in u8;
        data_out: out u16;
    }
    
    reflex r1 {
        match state {
            0 => {
                data_out = 100;
            }
            1 => {
                data_out = 200;
            }
            _ => {
                data_out = 0;
            }
        }
    }
}
"#;
    let expanded = expand_macros(source);
    println!("--- MATCH EXPANDED ---\n{}\n--- END ---", expanded);

    assert!(expanded.contains("guard auto_g_0"));
    assert!(expanded.contains("guard auto_g_1"));
    assert!(expanded.contains("on auto_g_0"));
    assert!(expanded.contains("on auto_g_1"));
    assert!(expanded.contains("on always"));

    let program = parse_mirr(&expanded).expect("Match expansion should parse successfully");
    assert!(!program.module.reflexes.is_empty());
}

#[test]
fn test_crossbar_match_expansion() {
    let source = r#"
module crossbar {
    signals {
        select_port: in u16;
        data_in_0: in u16;
        data_out_0: out u17;
    }

    reflex route {
        match select_port {
            0 => {
                data_out_0 = data_in_0;
            }
            _ => {
                data_out_0 = 0;
            }
        }
    }
}
"#;
    let expanded = expand_macros(source);
    println!("--- CROSSBAR EXPANDED ---\n{}\n--- END ---", expanded);
    let _program = parse_mirr(&expanded).expect("Crossbar expansion should parse successfully");
}

#[test]
fn test_temp_route_let() {
    let source = r#"
module crossbar {
    reflex route {
        on always_on {
            // Demonstrate ergonomic let signal assignment inline
            let temp_route_0: u16 = data_in_0;
        }
    }
}
"#;
    let expanded = expand_macros(source);
    println!("--- TEMP_ROUTE EXPANDED ---\n{}\n--- END ---", expanded);
}

#[test]
fn test_reflex_loop_expansion() {
    let source = r#"
module reflex_loop_expansion {
    signals {
        for i in 0..4 {
            data_in[i]: in bool;
        }
        for i in 0..4 {
            data_out[i]: out bool;
        }
    }

    reflex route {
        on always {
            for i in 0..4 {
                data_out[i] = data_in[i];
            }
        }
    }
}
"#;

    let expanded = expand_macros(source);
    println!("--- REFLEX LOOP EXPANDED ---\n{}\n--- END ---", expanded);

    // Verify unrolled signal names exist in the expanded source
    assert!(expanded.contains("signal data_in_0: in bool;"));
    assert!(expanded.contains("signal data_in_1: in bool;"));
    assert!(expanded.contains("signal data_in_2: in bool;"));
    assert!(expanded.contains("signal data_in_3: in bool;"));

    assert!(expanded.contains("signal data_out_0: out bool;"));
    assert!(expanded.contains("signal data_out_1: out bool;"));
    assert!(expanded.contains("signal data_out_2: out bool;"));
    assert!(expanded.contains("signal data_out_3: out bool;"));

    // Verify unrolled assignments exist in the expanded source
    assert!(expanded.contains("data_out_0 = data_in_0;"));
    assert!(expanded.contains("data_out_1 = data_in_1;"));
    assert!(expanded.contains("data_out_2 = data_in_2;"));
    assert!(expanded.contains("data_out_3 = data_in_3;"));

    // Ensure it parses successfully
    let program = parse_mirr(&expanded).expect("Expanded loop syntax should parse successfully");
    assert_eq!(program.module.name, "reflex_loop_expansion");
    assert_eq!(program.module.signals.len(), 8);
    assert_eq!(program.module.reflexes.len(), 1);
}
