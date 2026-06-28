#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]
use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn check(src: &str, is_err: bool, expected_err: Option<&str>) {
    let mut config = PipelineConfig::default();
    config.simplify = false;
    config.width = false;
    config.temporal = false;
    let full_src = format!("target profile {{ name: \"t\"; word_size: 64; }} {}", src);
    let result = run_pipeline(&full_src, &config);
    if is_err {
        let errs = result.expect_err("Expected type error");
        if let Some(msg) = expected_err {
            let err_str = errs.errors[0].to_string();
            assert!(err_str.contains(msg), "Expected error '{}', got '{}'", msg, err_str);
        }
    } else {
        assert!(result.is_ok(), "Expected valid typecheck, got: {:?}", result.err());
    }
}

#[test]
fn e601_mismatch_assignment() {
    check(
        "module test { signal y: out u16; reflex r { on always { y = true; } } }",
        true,
        Some("E601"),
    );
}
#[test]
fn e602_incompatible_binary() {
    check("module test { signal x: in bool; signal n: in u16; signal y: out bool; reflex r { on always { y = x + n; } } }", true, Some("E603"));
}
#[test]
fn e603_non_bool_guard() {
    check("module test { signal n: in u16; signal y: out bool; guard g { when n for 1 cycles; } reflex r { on g { y = true; } } }", true, Some("E601"));
}
#[test]
fn e604_invalid_array() {
    check("module test { signal arr: in u8[4]; signal y: out u8[5]; reflex r { on always { y = arr; } } }", true, Some("E601"));
}
#[test]
fn e607_mismatched_xor() {
    check("module test { signal n: in u16; signal b: in bool; signal y: out bool; reflex r { on always { y = n ^ b; } } }", true, Some("E607"));
}
#[test]
fn t1_bool_logic() {
    check("module test { signal x: in bool; signal y: out bool; reflex r { on always { y = x && !x; } } }", false, None);
}
#[test]
fn t2_unsigned_arithmetic() {
    check("module test { signal n: in u16; signal out_u16: out u16; reflex r { on always { out_u16 = n + 1; } } }", false, None);
}
#[test]
fn t4_comparison() {
    check("module test { signal n: in u16; signal y: out bool; reflex r { on always { y = n > 5; } } }", false, None);
}

#[test]
fn signed_widening_passes() {
    check("module test { signal si8: in i8; signal out_i16: out i16; reflex r { on always { out_i16 = si8; } } }", false, None);
}
#[test]
fn signed_narrowing_rejected() {
    check("module test { signal si16: in i16; signal out_i8: out i8; reflex r { on always { out_i8 = si16; } } }", true, Some("E601"));
}
#[test]
fn signed_unsigned_cross_rejected() {
    check("module test { signal su: in u16; signal si: in i16; signal out_i16: out i16; reflex r { on always { out_i16 = su + si; } } }", true, Some("E608"));
}
#[test]
fn signed_arithmetic_passes() {
    check("module test { signal a: in i16; signal b: in i8; signal out_i16: out i16; reflex r { on always { out_i16 = a - b; } } }", false, None);
}
#[test]
fn negate_unsigned_produces_signed() {
    check("module test { signal su: in u16; signal out_i32: out i32; reflex r { on always { out_i32 = -su; } } }", false, None);
}
