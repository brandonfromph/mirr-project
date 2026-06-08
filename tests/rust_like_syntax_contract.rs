#![forbid(unsafe_code)]

use mirrc::parser::parse_mirr;

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

    let program = parse_mirr(source).expect("Parser should accept Rust-like sugar after expansion");

    assert_eq!(program.module.name, "advanced_controller");
    assert_eq!(program.module.signals.len(), 3);
    assert_eq!(program.module.guards.len(), 2);
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

    let _program = parse_mirr(source).expect("Shorthand assignment should work");
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
    let _program = parse_mirr(source).expect("Property after reflex should work");
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
    let program = parse_mirr(source).expect("Let binding expansion should parse successfully");
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
    let program = parse_mirr(source).expect("Match expansion should parse successfully");
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
    let _program = parse_mirr(source).expect("Crossbar expansion should parse successfully");
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
    let _program = parse_mirr(source).expect("temp_route_let should parse successfully");
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

    let program = parse_mirr(source).expect("Expanded loop syntax should parse successfully");
    assert_eq!(program.module.name, "reflex_loop_expansion");
    assert_eq!(program.module.signals.len(), 8);
    assert_eq!(program.module.reflexes.len(), 1);
}
