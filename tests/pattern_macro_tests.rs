#[test]
fn test_pattern_parameter_dependent_for_loop_expansion() {
    use nasa_rust_project::parse_mirr;
    let src = r#"
        def p(N: u16) {
            reflect {
                for i in 0..${N} {
                    signal s_${i}: bool;
                }
            }
        }

        module m {
            // Correct syntax: call pattern 'p' with argument '2'
            p(2);
        }
    "#;

    let result = parse_mirr(src);
    assert!(result.is_ok(), "Expansion failed: {:?}", result.err());

    let program = result.unwrap();
    let pat = &program.patterns[0];
    assert_eq!(pat.body.statements.len(), 1, "Should contain one for-loop statement");

    use nasa_rust_project::ast::macro_nodes::ModuleMacroStmt;
    assert!(matches!(pat.body.statements[0], ModuleMacroStmt::ForLoop { .. }));
}
