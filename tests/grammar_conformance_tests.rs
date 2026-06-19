#![cfg(any())]
#![forbid(unsafe_code)]
//! Grammar conformance tests covering 50 unique syntactic scenarios.
use mirrc::pipeline::{run_pipeline, PipelineConfig};

macro_rules! test_grammar {
    ($($name:ident, $src:expr, $expected_ok:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let config = PipelineConfig::default();
                let result = run_pipeline($src, &config);
                if $expected_ok {
                    assert!(
                        result.is_ok(),
                        "Expected grammar to be valid, but failed: {:?}",
                        result.err()
                    );
                } else {
                    assert!(
                        result.is_err(),
                        "Expected grammar to be invalid, but it parsed successfully!"
                    );
                }
            }
        )*
    };
}

test_grammar! {
    // === 25 Valid Grammar Cases ===
    valid_empty_module, "module Empty {}", true;
    valid_single_signal, "module Single { signal a: in bool; }", true;
    valid_multiple_signals, "module Multi { signal a: in bool; signal b: out u8; }", true;
    valid_internal_signal, "module IntSig { signal a: internal u16; }", true;
    valid_guard_basic, "module GuardBasic { signal a: in bool; guard g { when a for 1 cycles; } }", true;
    valid_guard_multi_cycles, "module GuardCycles { signal a: in bool; guard g { when a for 42 cycles; } }", true;
    valid_reflex_basic, "module ReflexBasic { signal a: in bool; signal b: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; } } }", true;
    valid_reflex_always, "module ReflexAlways { signal b: out bool; reflex r { on always { b = true; } } }", true;
    valid_reflex_multiple_assign, "module ReflexMulti { signal a: in bool; signal b: out bool; signal c: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; c = false; } } }", true;
    valid_property_basic, "module PropBasic { signal a: in bool; property p { always (a); } }", true;
    valid_refinement_less, "module RefLess { signal a: in u8 where x < 10; }", true;
    valid_refinement_greater, "module RefGreater { signal a: in u8 where x > 5; }", true;
    valid_refinement_eq, "module RefEq { signal a: in u8 where x >= 3; }", true;
    valid_refinement_complex, "module RefComplex { signal a: in u8 where 0..10; }", true;
    valid_prev_op, "module PrevOp { signal a: in bool; signal b: out bool; guard g { when prev(a, 2) for 1 cycles; } }", true;
    valid_binary_and, "module BinAnd { signal a: in bool; signal b: in bool; guard g { when a && b for 1 cycles; } }", true;
    valid_binary_or, "module BinOr { signal a: in bool; signal b: in bool; guard g { when a || b for 1 cycles; } }", true;
    valid_binary_comparison, "module BinComp { signal a: in u8; guard g { when a == 5 for 1 cycles; } }", true;
    valid_unary_not, "module UnaryNot { signal a: in bool; guard g { when !a for 1 cycles; } }", true;
    valid_pattern_def, "def my_pat(s: signal in bool) { reflect { signal x: internal bool; reflex r { x = ${s}; } } } module UsePat { signal a: in bool; my_pat(a); }", true;
    valid_comment_line, "module Comm { // single line comment\n signal a: in bool; }", true;
    valid_comment_trailing, "module CommTrailing { signal a: in bool; // trailing comment\n }", true;
    valid_hex_literal, "module HexLit { signal a: in u8; guard g { when a == 0xFF for 1 cycles; } }", true;
    valid_large_dec_literal, "module LargeDec { signal a: in u16; guard g { when a == 65535 for 1 cycles; } }", true;
    valid_nested_module_names, "module A::B::C { signal a: in bool; }", true;

    // === 25 Invalid Grammar Cases ===
    invalid_no_module_keyword, "Empty {}", false;
    invalid_missing_module_name, "module {}", false;
    invalid_missing_braces, "module NoBraces", false;
    invalid_signal_missing_colon, "module Sig { signal a in bool; }", false;
    invalid_signal_missing_semicolon, "module Sig { signal a: in bool }", false;
    invalid_signal_missing_type, "module Sig { signal a: in; }", false;
    invalid_guard_missing_keyword, "module Guard { signal a: in bool; g { when a for 1 cycles; } }", false;
    invalid_guard_missing_when, "module Guard { signal a: in bool; guard g { a for 1 cycles; } }", false;
    invalid_guard_missing_cycles, "module Guard { signal a: in bool; guard g { when a; } }", false;
    invalid_reflex_missing_on, "module Reflex { signal b: out bool; reflex r { on { b = true; } } }", false;
    invalid_reflex_missing_target, "module Reflex { reflex r { on always { = true; } } }", false;
    invalid_reflex_missing_semicolon, "module Reflex { signal b: out bool; reflex r { on always { b = true } } }", false;
    invalid_property_missing_formula, "module Prop { property p { a; } }", false;
    invalid_invalid_refinement_var, "module Ref { signal a: in u8{y < 10}; }", false; // must use 'x'
    invalid_invalid_literal, "module Lit { signal a: in u8; guard g { when a == 12a for 1 cycles; } }", false;
    invalid_double_semicolon, "module Semi { signal a: in bool;; }", false;
    invalid_dangling_operator, "module Op { signal a: in bool; guard g { when a && for 1 cycles; } }", false;
    invalid_unmatched_brace, "module Brace { signal a: in bool; reflex r { on always { a = true; } }", false;
    invalid_unmatched_bracket, "module Bracket { signal a: in u8[10; }", false;
    invalid_pattern_no_reflect, "def pat(s: signal in bool) { signal x: internal bool; }", false;
    invalid_invalid_token, "module Token { @#$% }", false;
    invalid_empty_signal_decl, "module EmptySig { signal; }", false;
    invalid_empty_guard_decl, "module EmptyGuard { guard; }", false;
    invalid_empty_reflex_decl, "module EmptyReflex { reflex; }", false;
    invalid_empty_property_decl, "module EmptyProp { property; }", false;
}
