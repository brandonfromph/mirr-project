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
    assert_eq!(program.module.guards.len(), 1);

    // The if/else-if logic should be lowered to separate reflexes or combined with OR logic if possible.
    // For now, we expect two separate reflexes or a single reflex with multiple trigger guards.
    assert!(program.module.reflexes.len() >= 1);
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
