// Bug Hunt #11: Check if the AST parser handles extremely deep nested properties without a stack overflow.
use mirrc::parser::parse_mirr;
#[test]
fn test_deep_nesting_stack_overflow() {
    let mut nested_expr = "a".to_string();
    for _ in 0..5000 {
        nested_expr = format!("({} && b)", nested_expr);
    }
    let input = format!("module test {{ property p {{ always {} ; }} }}", nested_expr);
    let result = parse_mirr(&input);
    // It should either parse OK or fail with a controlled [E...] error, NOT a stack overflow (SIGABRT/SIGSEGV).
    // Note: Rust test harnesses catch panics, but stack overflows abort the process.
    if let Err(e) = result {
        println!("Expected failure gracefully handled: {:?}", e);
    }
}
