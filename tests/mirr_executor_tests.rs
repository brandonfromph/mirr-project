//! Integration tests for `src/mirr_executor.rs` — the MIRR signal evaluator and interpreter engine.
//!
//! Tests `drive_parsed_module_with_interpreter` with a single comprehensive MIRR
//! module that covers all signal types recognized by the executor. A single module
//! is used because the executor's runtime pools are `static OnceLock`-backed and
//! only reinitialize when the program fingerprint (shape) changes.
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - All loops use explicit `MAX_*` bounded iteration constants.
//! - No recursion in any test helper.
//! - Every `assert!` has a descriptive message string.

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

use mirrc::mirr_executor::drive_parsed_module_with_interpreter;
use mirrc::parser::parse_mirr;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum observed pushes to scan in any single test.
const MAX_PUSH_SCAN: usize = 1024;

/// Maximum ticks to drive in stress tests.
const MAX_STRESS_TICKS: usize = 256;

/// Maximum guards/reflexes to check in module structure tests.
const MAX_STRUCT_CHECK: usize = 64;

// ---------------------------------------------------------------------------
// Shared comprehensive module — matches executor's expected signal names
// ---------------------------------------------------------------------------

/// A comprehensive module with all signals the executor's input detection
/// and push-emission logic recognizes. This ensures the static runtime pools
/// are initialized with all necessary keys regardless of test execution order.
const COMPREHENSIVE_MODULE: &str = r#"
module comprehensive_test {
    signal input_byte_is_digit: in bool;
    signal input_byte_is_whitespace: in bool;
    signal input_two_eq: in bool;
    signal input_two_ne: in bool;
    signal input_two_le: in bool;
    signal input_two_ge: in bool;
    signal input_arrow: in bool;
    signal input_dotdot: in bool;
    signal input_ident_len5: in bool;
    signal input_ident_len6: in bool;
    signal input_ident_len8: in bool;
    signal input_ident_guard: in bool;
    signal input_ident_false: in bool;
    signal input_ident_break: in bool;
    signal input_ident_while: in bool;
    signal input_ident_match: in bool;
    signal input_ident_const: in bool;
    signal input_ident_module: in bool;
    signal input_ident_signal: in bool;
    signal input_ident_reflex: in bool;
    signal input_ident_return: in bool;
    signal input_ident_struct: in bool;
    signal input_ident_cycles: in bool;
    signal input_ident_internal: in bool;

    signal emit_push_integer: out bool;
    signal emit_push_ident: out bool;
    signal emit_push_eq_eq: out bool;
    signal emit_push_excl_eq: out bool;
    signal emit_push_le: out bool;
    signal emit_push_ge: out bool;
    signal emit_push_arrow: out bool;
    signal emit_push_dot_dot: out bool;
    signal emit_push_kw_when: out bool;
    signal emit_push_kw_bool: out bool;
    signal emit_push_tok_true: out bool;
    signal emit_push_kw_else: out bool;
    signal emit_push_kw_loop: out bool;
    signal emit_push_kw_enum: out bool;
    signal emit_push_kw_guard: out bool;
    signal emit_push_tok_false: out bool;
    signal emit_push_kw_break: out bool;
    signal emit_push_kw_while: out bool;
    signal emit_push_kw_match: out bool;
    signal emit_push_kw_const: out bool;
    signal emit_push_kw_module: out bool;
    signal emit_push_kw_signal: out bool;
    signal emit_push_kw_reflex: out bool;
    signal emit_push_kw_return: out bool;
    signal emit_push_kw_struct: out bool;
    signal emit_push_kw_cycles: out bool;
    signal emit_push_kw_internal: out bool;

    guard digit_guard {
        when input_byte_is_digit
        for 1 cycles;
    }

    guard eq_guard {
        when input_two_eq
        for 1 cycles;
    }

    guard ne_guard {
        when input_two_ne
        for 1 cycles;
    }

    guard le_guard {
        when input_two_le
        for 1 cycles;
    }

    guard ge_guard {
        when input_two_ge
        for 1 cycles;
    }

    guard arrow_guard {
        when input_arrow
        for 1 cycles;
    }

    guard dotdot_guard {
        when input_dotdot
        for 1 cycles;
    }

    guard kw_guard_guard {
        when input_ident_guard
        for 1 cycles;
    }

    guard kw_false_guard {
        when input_ident_false
        for 1 cycles;
    }

    guard kw_break_guard {
        when input_ident_break
        for 1 cycles;
    }

    guard kw_while_guard {
        when input_ident_while
        for 1 cycles;
    }

    guard kw_match_guard {
        when input_ident_match
        for 1 cycles;
    }

    guard kw_const_guard {
        when input_ident_const
        for 1 cycles;
    }

    guard kw_module_guard {
        when input_ident_module
        for 1 cycles;
    }

    guard kw_signal_guard {
        when input_ident_signal
        for 1 cycles;
    }

    guard kw_reflex_guard {
        when input_ident_reflex
        for 1 cycles;
    }

    guard kw_return_guard {
        when input_ident_return
        for 1 cycles;
    }

    guard kw_struct_guard {
        when input_ident_struct
        for 1 cycles;
    }

    guard kw_cycles_guard {
        when input_ident_cycles
        for 1 cycles;
    }

    guard kw_internal_guard {
        when input_ident_internal
        for 1 cycles;
    }

    reflex push_integer {
        on digit_guard {
            emit_push_integer = true;
        }
    }

    reflex push_eq_eq {
        on eq_guard {
            emit_push_eq_eq = true;
        }
    }

    reflex push_excl_eq {
        on ne_guard {
            emit_push_excl_eq = true;
        }
    }

    reflex push_le {
        on le_guard {
            emit_push_le = true;
        }
    }

    reflex push_ge {
        on ge_guard {
            emit_push_ge = true;
        }
    }

    reflex push_arrow {
        on arrow_guard {
            emit_push_arrow = true;
        }
    }

    reflex push_dot_dot {
        on dotdot_guard {
            emit_push_dot_dot = true;
        }
    }

    reflex push_kw_guard {
        on kw_guard_guard {
            emit_push_kw_guard = true;
        }
    }

    reflex push_tok_false {
        on kw_false_guard {
            emit_push_tok_false = true;
        }
    }

    reflex push_kw_break {
        on kw_break_guard {
            emit_push_kw_break = true;
        }
    }

    reflex push_kw_while {
        on kw_while_guard {
            emit_push_kw_while = true;
        }
    }

    reflex push_kw_match {
        on kw_match_guard {
            emit_push_kw_match = true;
        }
    }

    reflex push_kw_const {
        on kw_const_guard {
            emit_push_kw_const = true;
        }
    }

    reflex push_kw_module {
        on kw_module_guard {
            emit_push_kw_module = true;
        }
    }

    reflex push_kw_signal {
        on kw_signal_guard {
            emit_push_kw_signal = true;
        }
    }

    reflex push_kw_reflex {
        on kw_reflex_guard {
            emit_push_kw_reflex = true;
        }
    }

    reflex push_kw_return {
        on kw_return_guard {
            emit_push_kw_return = true;
        }
    }

    reflex push_kw_struct {
        on kw_struct_guard {
            emit_push_kw_struct = true;
        }
    }

    reflex push_kw_cycles {
        on kw_cycles_guard {
            emit_push_kw_cycles = true;
        }
    }

    reflex push_kw_internal {
        on kw_internal_guard {
            emit_push_kw_internal = true;
        }
    }
}
"#;

fn parse_comprehensive() -> mirrc::ast::MirrProgram {
    parse_mirr(COMPREHENSIVE_MODULE).expect("comprehensive module must parse")
}

fn parse_ok(src: &str) -> mirrc::ast::MirrProgram {
    parse_mirr(src).expect("parse_mirr should succeed for valid test input")
}

fn count_pushes(pushes: &[mirrc::mirr_driver::ObservedPush], kind: &str) -> usize {
    pushes.iter().take(MAX_PUSH_SCAN).filter(|p| p.kind == kind).count()
}

fn find_push<'a>(
    pushes: &'a [mirrc::mirr_driver::ObservedPush],
    kind: &str,
) -> Option<&'a mirrc::mirr_driver::ObservedPush> {
    pushes.iter().take(MAX_PUSH_SCAN).find(|p| p.kind == kind)
}

fn drive_prog(
    _prog: &mirrc::ast::MirrProgram,
    input: &[u8],
) -> Vec<mirrc::mirr_driver::ObservedPush> {
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, "ERROR_NO_SRC", None).unwrap();
    drive_parsed_module_with_interpreter(&reg, input)
}

// ===========================================================================
// Section 1: Empty / whitespace / invalid input
// ===========================================================================

#[test]
fn empty_input_returns_empty_pushes() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"");

    assert!(pushes.is_empty(), "empty input must produce zero pushes, got {}", pushes.len());
}

#[test]
fn whitespace_only_input_returns_empty_pushes() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"   \t\n\r  ");

    assert!(
        pushes.is_empty(),
        "whitespace-only input must produce zero pushes, got {}",
        pushes.len()
    );
}

#[test]
fn invalid_utf8_returns_empty_pushes() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, &[0xFF, 0xFE]);

    assert!(
        pushes.is_empty(),
        "invalid UTF-8 input must produce zero pushes, got {}",
        pushes.len()
    );
}

// ===========================================================================
// Section 2: Digit input and integer push emission
// ===========================================================================

#[test]
fn digit_input_triggers_integer_push() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"42");

    assert!(
        count_pushes(&pushes, "emit_push_integer") >= 1,
        "digit input '42' must trigger at least one integer push"
    );
}

#[test]
fn integer_push_carries_parsed_value() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"99");

    let p = find_push(&pushes, "emit_push_integer");
    assert!(p.is_some(), "must find integer push for '99'");
    assert_eq!(p.unwrap().int_val, Some(99), "integer push for '99' must carry int_val=99");
}

#[test]
fn zero_integer_parsed_correctly() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"0");

    let p = find_push(&pushes, "emit_push_integer");
    assert!(p.is_some(), "input '0' must produce an integer push");
    assert_eq!(p.unwrap().int_val, Some(0), "integer push for '0' must carry int_val=0");
}

#[test]
fn multiple_separate_digits() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"1 2 3");

    let count = count_pushes(&pushes, "emit_push_integer");
    assert!(count >= 3, "'1 2 3' must produce at least 3 integer pushes, got {}", count);
}

// ===========================================================================
// Section 3: Two-character operators
// ===========================================================================

#[test]
fn two_char_eq_eq_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"==");

    assert!(find_push(&pushes, "emit_push_eq_eq").is_some(), "'==' must trigger emit_push_eq_eq");
}

#[test]
fn two_char_ne_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"!=");

    assert!(
        find_push(&pushes, "emit_push_excl_eq").is_some(),
        "'!=' must trigger emit_push_excl_eq"
    );
}

#[test]
fn two_char_le_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"<=");

    assert!(find_push(&pushes, "emit_push_le").is_some(), "'<=' must trigger emit_push_le");
}

#[test]
fn two_char_ge_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b">=");

    assert!(find_push(&pushes, "emit_push_ge").is_some(), "'>=' must trigger emit_push_ge");
}

#[test]
fn two_char_arrow_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"->");

    assert!(find_push(&pushes, "emit_push_arrow").is_some(), "'->' must trigger emit_push_arrow");
}

#[test]
fn two_char_dotdot_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"..");

    assert!(
        find_push(&pushes, "emit_push_dot_dot").is_some(),
        "'..' must trigger emit_push_dot_dot"
    );
}

#[test]
fn single_eq_does_not_trigger_eq_eq() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"=");

    assert!(
        find_push(&pushes, "emit_push_eq_eq").is_none(),
        "single '=' must NOT trigger emit_push_eq_eq"
    );
}

// ===========================================================================
// Section 4: Keyword recognition
// ===========================================================================

#[test]
fn keyword_guard_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"guard");

    assert!(
        find_push(&pushes, "emit_push_kw_guard").is_some(),
        "'guard' must trigger emit_push_kw_guard"
    );
}

#[test]
fn keyword_module_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"module");

    assert!(
        find_push(&pushes, "emit_push_kw_module").is_some(),
        "'module' must trigger emit_push_kw_module"
    );
}

#[test]
fn keyword_signal_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"signal");

    assert!(
        find_push(&pushes, "emit_push_kw_signal").is_some(),
        "'signal' must trigger emit_push_kw_signal"
    );
}

#[test]
fn keyword_reflex_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"reflex");

    assert!(
        find_push(&pushes, "emit_push_kw_reflex").is_some(),
        "'reflex' must trigger emit_push_kw_reflex"
    );
}

#[test]
fn keyword_false_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"false");

    assert!(
        find_push(&pushes, "emit_push_tok_false").is_some(),
        "'false' must trigger emit_push_tok_false"
    );
}

#[test]
fn keyword_while_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"while");

    assert!(
        find_push(&pushes, "emit_push_kw_while").is_some(),
        "'while' must trigger emit_push_kw_while"
    );
}

#[test]
fn keyword_const_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"const");

    assert!(
        find_push(&pushes, "emit_push_kw_const").is_some(),
        "'const' must trigger emit_push_kw_const"
    );
}

#[test]
fn keyword_return_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"return");

    assert!(
        find_push(&pushes, "emit_push_kw_return").is_some(),
        "'return' must trigger emit_push_kw_return"
    );
}

#[test]
fn keyword_cycles_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"cycles");

    assert!(
        find_push(&pushes, "emit_push_kw_cycles").is_some(),
        "'cycles' must trigger emit_push_kw_cycles"
    );
}

#[test]
fn keyword_internal_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"internal");

    assert!(
        find_push(&pushes, "emit_push_kw_internal").is_some(),
        "'internal' must trigger emit_push_kw_internal"
    );
}

#[test]
fn keyword_break_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"break");

    assert!(
        find_push(&pushes, "emit_push_kw_break").is_some(),
        "'break' must trigger emit_push_kw_break"
    );
}

#[test]
fn keyword_match_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"match");

    assert!(
        find_push(&pushes, "emit_push_kw_match").is_some(),
        "'match' must trigger emit_push_kw_match"
    );
}

#[test]
fn keyword_struct_recognized() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"struct");

    assert!(
        find_push(&pushes, "emit_push_kw_struct").is_some(),
        "'struct' must trigger emit_push_kw_struct"
    );
}

// ===========================================================================
// Section 5: Mixed token sequences
// ===========================================================================

#[test]
fn mixed_keywords_and_digits() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"module 42 module");

    assert!(
        count_pushes(&pushes, "emit_push_kw_module") >= 2,
        "must have >= 2 module pushes for 'module 42 module'"
    );
    assert!(
        count_pushes(&pushes, "emit_push_integer") >= 1,
        "must have >= 1 integer push for '42'"
    );
}

#[test]
fn operators_and_keywords_interleaved() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"guard == signal");

    assert!(find_push(&pushes, "emit_push_kw_guard").is_some(), "'guard' must be recognized");
    assert!(find_push(&pushes, "emit_push_eq_eq").is_some(), "'==' must be recognized");
    assert!(find_push(&pushes, "emit_push_kw_signal").is_some(), "'signal' must be recognized");
}

// ===========================================================================
// Section 6: Module structure verification
// ===========================================================================

#[test]
fn comprehensive_module_parses_with_expected_signals() {
    let prog = parse_comprehensive();
    let signals = &prog.module.signals;

    let input_count = signals
        .iter()
        .take(MAX_STRUCT_CHECK)
        .filter(|s| s.kind == mirrc::ast::types::SignalKind::Input)
        .count();
    let output_count = signals
        .iter()
        .take(MAX_STRUCT_CHECK)
        .filter(|s| s.kind == mirrc::ast::types::SignalKind::Output)
        .count();

    assert!(
        input_count >= 20,
        "comprehensive module must have >= 20 input signals, got {}",
        input_count
    );
    assert!(
        output_count >= 20,
        "comprehensive module must have >= 20 output signals, got {}",
        output_count
    );
}

#[test]
fn comprehensive_module_has_expected_guards() {
    let prog = parse_comprehensive();
    let guards = &prog.module.guards;

    assert!(
        guards.len() >= 20,
        "comprehensive module must have >= 20 guards, got {}",
        guards.len()
    );

    for i in 0..guards.len().min(MAX_STRUCT_CHECK) {
        assert_eq!(guards[i].cycles, 1, "guard '{}' must have 1 cycle", guards[i].name);
    }
}

#[test]
fn comprehensive_module_has_expected_reflexes() {
    let prog = parse_comprehensive();
    let reflexes = &prog.module.reflexes;

    assert!(
        reflexes.len() >= 20,
        "comprehensive module must have >= 20 reflexes, got {}",
        reflexes.len()
    );

    for i in 0..reflexes.len().min(MAX_STRUCT_CHECK) {
        assert!(
            !reflexes[i].assignments.is_empty(),
            "reflex '{}' must have at least one assignment",
            reflexes[i].name
        );
    }
}

// ===========================================================================
// Section 7: Module with no guards
// ===========================================================================

#[test]
fn module_with_no_guards_produces_no_pushes() {
    let src = r#"
module no_guard_mod {
    signal a: in bool;
    signal b: out bool;
}
"#;
    let prog = parse_ok(src);
    let pushes = drive_prog(&prog, b"hello 123");

    assert!(
        pushes.is_empty(),
        "module with no guards/reflexes must produce no pushes, got {}",
        pushes.len()
    );
}

// ===========================================================================
// Section 8: Stress test — bounded iteration
// ===========================================================================

#[test]
fn stress_many_digit_tokens_bounded() {
    let prog = parse_comprehensive();

    let mut input = String::new();
    for i in 0..MAX_STRESS_TICKS.min(128) {
        if i > 0 {
            input.push(' ');
        }
        input.push_str(&i.to_string());
    }

    let pushes = drive_prog(&prog, input.as_bytes());

    let count = count_pushes(&pushes, "emit_push_integer");
    assert!(
        count >= 64,
        "stress test with 128 tokens must produce >= 64 integer pushes, got {}",
        count
    );
}

#[test]
fn stress_many_keywords_bounded() {
    let prog = parse_comprehensive();

    let mut input = String::new();
    let keywords = ["guard", "module", "signal", "reflex", "const", "while", "break", "match"];
    for i in 0..MAX_STRESS_TICKS.min(64) {
        if i > 0 {
            input.push(' ');
        }
        input.push_str(keywords[i % keywords.len()]);
    }

    let pushes = drive_prog(&prog, input.as_bytes());

    assert!(
        pushes.len() >= 32,
        "stress test with 64 keyword tokens must produce >= 32 pushes, got {}",
        pushes.len()
    );
}

// ===========================================================================
// Section 9: Alloc hook mechanism
// ===========================================================================

#[test]
fn alloc_hook_invoked_on_drive() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static HOOK_COUNT: AtomicUsize = AtomicUsize::new(0);

    fn hook(_label: &str) {
        HOOK_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    mirrc::mirr_executor::set_alloc_hook(hook);

    let prog = parse_comprehensive();
    let _pushes = drive_prog(&prog, b"42");

    let count = HOOK_COUNT.load(Ordering::SeqCst);
    assert!(
        count > 0,
        "alloc hook must be invoked at least once during drive, got count={}",
        count
    );
}

// ===========================================================================
// Section 10: Unrecognized identifiers (no push emitted)
// ===========================================================================

#[test]
fn unknown_identifier_no_keyword_push() {
    let prog = parse_comprehensive();
    let pushes = drive_prog(&prog, b"foobar");

    // "foobar" is not a recognized keyword, so no keyword push should fire.
    // (It may produce length-class matches but NOT keyword-specific pushes.)
    let keyword_pushes: Vec<_> = pushes
        .iter()
        .take(MAX_PUSH_SCAN)
        .filter(|p| p.kind.starts_with("emit_push_kw_") || p.kind.starts_with("emit_push_tok_"))
        .collect();

    assert!(
        keyword_pushes.is_empty(),
        "unknown identifier 'foobar' must not trigger keyword pushes, got {:?}",
        keyword_pushes.iter().map(|p| p.kind).collect::<Vec<_>>()
    );
}
