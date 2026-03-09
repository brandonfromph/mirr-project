//! Tests for source span propagation through the parser.
//!
//! Verifies that parsed AST nodes carry correct line-level spans and
//! that parse errors include span information.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::span::Span;

#[test]
fn signal_decl_has_full_line_span() {
    let src = "module m {\n    signal x: in bool;\n}\n";
    let program = parse_mirr(src).unwrap();
    let sig = &program.module.signals[0];
    assert_eq!(sig.name, "x");
    // "signal x: in bool;" is on line index 1 (0-based).
    let span = sig.span.expect("signal should have a span");
    assert_eq!(span.start_line, 1);
    assert_eq!(span.end_line, 1);
    assert_eq!(span.start_col, 0);
}

#[test]
fn guard_has_multi_line_span() {
    let src = "\
module m {
    signal a: in bool;
    guard g {
        when a
        for 10 cycles;
    }
}
";
    let program = parse_mirr(src).unwrap();
    let guard = &program.module.guards[0];
    assert_eq!(guard.name, "g");
    let span = guard.span.expect("guard should have a span");
    // guard starts at "guard g {" (line 2) and ends at "}" (line 5).
    assert_eq!(span.start_line, 2);
    assert_eq!(span.end_line, 5);
}

#[test]
fn reflex_has_multi_line_span() {
    let src = "\
module m {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
";
    let program = parse_mirr(src).unwrap();
    let reflex = &program.module.reflexes[0];
    assert_eq!(reflex.name, "r");
    let span = reflex.span.expect("reflex should have a span");
    // reflex starts at "reflex r {" (line 7) and ends at outer "}" (line 11).
    assert_eq!(span.start_line, 7);
    assert_eq!(span.end_line, 11);
}

#[test]
fn property_has_multi_line_span() {
    let src = "\
module m {
    signal a: in bool;
    property p {
        always (a)
    }
}
";
    let program = parse_mirr(src).unwrap();
    let prop = &program.module.properties[0];
    assert_eq!(prop.name, "p");
    let span = prop.span.expect("property should have a span");
    assert_eq!(span.start_line, 2);
    assert_eq!(span.end_line, 4);
}

#[test]
fn module_has_span_covering_entire_block() {
    let src = "\
module m {
    signal x: in bool;
}
";
    let program = parse_mirr(src).unwrap();
    let span = program.module.span.expect("module should have a span");
    // Module starts at "module m {" (line 0) and ends at "}" (line 2).
    assert_eq!(span.start_line, 0);
    assert_eq!(span.end_line, 2);
}

#[test]
fn assignment_inside_reflex_has_span() {
    let src = "\
module m {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
";
    let program = parse_mirr(src).unwrap();
    let assign = &program.module.reflexes[0].assignments[0];
    let span = assign.span.expect("assignment should have a span");
    // "b = true;" is on line 9.
    assert_eq!(span.start_line, 9);
    assert_eq!(span.end_line, 9);
}

#[test]
fn parse_error_has_span() {
    let src = "\
module m {
    signal x: in bool;
    garbage line here
}
";
    let err = parse_mirr(src).unwrap_err();
    let span = err.span().expect("parse error should have a span");
    // "garbage line here" is on line 2.
    assert_eq!(span.start_line, 2);
}

#[test]
fn semantic_error_has_span_from_node() {
    // E201 duplicate signal — the second signal should carry a span.
    let src = "\
module m {
    signal x: in bool;
    signal x: in bool;
}
";
    let program = parse_mirr(src).unwrap();
    let errs = nasa_rust_project::validate_module(&program.module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("E201"), "expected E201 duplicate signal error, got: {msg}");
    let span = err.span().expect("semantic error should have a span from the duplicate signal");
    // The second "signal x" is on line 2.
    assert_eq!(span.start_line, 2);
}

#[test]
fn span_merge_works() {
    let a = Span::single_line(2, 5, 10);
    let b = Span::single_line(4, 0, 15);
    let merged = a.merge(b);
    assert_eq!(merged.start_line, 2);
    assert_eq!(merged.start_col, 5);
    assert_eq!(merged.end_line, 4);
    assert_eq!(merged.end_col, 15);
}
