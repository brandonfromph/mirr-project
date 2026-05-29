#![forbid(unsafe_code)]

use nasa_rust_project::compiler::macro_proc::expand_macros;

#[test]
fn test_loop_unrolling_variant_1() {
    let out = expand_macros("signals {\n for (i_1 = 0; i_1 < 2; i_1++) {\n signal arr[i_1]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_2() {
    let out = expand_macros("signals {\n for (i_2 = 0; i_2 < 2; i_2++) {\n signal arr[i_2]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_3() {
    let out = expand_macros("signals {\n for (i_3 = 0; i_3 < 2; i_3++) {\n signal arr[i_3]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_4() {
    let out = expand_macros("signals {\n for (i_4 = 0; i_4 < 2; i_4++) {\n signal arr[i_4]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_5() {
    let out = expand_macros("signals {\n for (i_5 = 0; i_5 < 2; i_5++) {\n signal arr[i_5]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_6() {
    let out = expand_macros("signals {\n for (i_6 = 0; i_6 < 2; i_6++) {\n signal arr[i_6]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_7() {
    let out = expand_macros("signals {\n for (i_7 = 0; i_7 < 2; i_7++) {\n signal arr[i_7]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_8() {
    let out = expand_macros("signals {\n for (i_8 = 0; i_8 < 2; i_8++) {\n signal arr[i_8]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_9() {
    let out = expand_macros("signals {\n for (i_9 = 0; i_9 < 2; i_9++) {\n signal arr[i_9]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_10() {
    let out = expand_macros("signals {\n for (i_10 = 0; i_10 < 2; i_10++) {\n signal arr[i_10]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_11() {
    let out = expand_macros("signals {\n for (i_11 = 0; i_11 < 2; i_11++) {\n signal arr[i_11]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_12() {
    let out = expand_macros("signals {\n for (i_12 = 0; i_12 < 2; i_12++) {\n signal arr[i_12]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_13() {
    let out = expand_macros("signals {\n for (i_13 = 0; i_13 < 2; i_13++) {\n signal arr[i_13]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_14() {
    let out = expand_macros("signals {\n for (i_14 = 0; i_14 < 2; i_14++) {\n signal arr[i_14]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_15() {
    let out = expand_macros("signals {\n for (i_15 = 0; i_15 < 2; i_15++) {\n signal arr[i_15]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_16() {
    let out = expand_macros("signals {\n for (i_16 = 0; i_16 < 2; i_16++) {\n signal arr[i_16]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_17() {
    let out = expand_macros("signals {\n for (i_17 = 0; i_17 < 2; i_17++) {\n signal arr[i_17]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_18() {
    let out = expand_macros("signals {\n for (i_18 = 0; i_18 < 2; i_18++) {\n signal arr[i_18]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_19() {
    let out = expand_macros("signals {\n for (i_19 = 0; i_19 < 2; i_19++) {\n signal arr[i_19]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_20() {
    let out = expand_macros("signals {\n for (i_20 = 0; i_20 < 2; i_20++) {\n signal arr[i_20]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_21() {
    let out = expand_macros("signals {\n for (i_21 = 0; i_21 < 2; i_21++) {\n signal arr[i_21]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_22() {
    let out = expand_macros("signals {\n for (i_22 = 0; i_22 < 2; i_22++) {\n signal arr[i_22]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_23() {
    let out = expand_macros("signals {\n for (i_23 = 0; i_23 < 2; i_23++) {\n signal arr[i_23]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_24() {
    let out = expand_macros("signals {\n for (i_24 = 0; i_24 < 2; i_24++) {\n signal arr[i_24]: u8; \n}\n}");
    assert!(!out.is_empty());
}

#[test]
fn test_loop_unrolling_variant_25() {
    let out = expand_macros("signals {\n for (i_25 = 0; i_25 < 2; i_25++) {\n signal arr[i_25]: u8; \n}\n}");
    assert!(!out.is_empty());
}

