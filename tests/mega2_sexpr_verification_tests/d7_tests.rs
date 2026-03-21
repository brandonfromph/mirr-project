use super::*;

// ===========================================================================
// D7: parser_depth_limits (10 tests)
// ===========================================================================

#[test]
fn test_d7_depth_1_ok() {
    let r = parse_sexpr("(a)");
    assert!(r.is_ok(), "Depth 1 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_10_ok() {
    let open: String = "(".repeat(10);
    let close: String = ")".repeat(10);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 10 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_30_ok() {
    let open: String = "(".repeat(30);
    let close: String = ")".repeat(30);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 30 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_max_ok() {
    // Parser checks `current_depth >= MAX_SEXPR_DEPTH`, so the deepest
    // valid nesting is MAX_SEXPR_DEPTH - 1 open parens.
    let open: String = "(".repeat(MAX_SEXPR_DEPTH - 1);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH - 1);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth MAX-1 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_max_plus_1_fails() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 1);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 1);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth MAX+1 should fail");
}

#[test]
fn test_d7_depth_max_plus_10_fails() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 10);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 10);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth MAX+10 should fail");
}

#[test]
fn test_d7_flat_list_at_depth_1() {
    // A wide but shallow list should be fine
    let mut items = String::new();
    let mut i = 0;
    while i < 100 {
        items.push_str(&format!("x{i} "));
        i += 1;
    }
    let input = format!("({items})");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Wide flat list should parse: {:?}", r.err());
}

#[test]
fn test_d7_nested_lists_at_same_depth() {
    // ((a) (b) (c)) — depth 2, not deep
    let r = parse_sexpr("((a) (b) (c))");
    assert!(r.is_ok());
}

#[test]
fn test_d7_depth_50_ok() {
    let open: String = "(".repeat(50);
    let close: String = ")".repeat(50);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 50 (< MAX 64) should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_100_fails() {
    let open: String = "(".repeat(100);
    let close: String = ")".repeat(100);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth 100 (> MAX 64) should fail");
}

