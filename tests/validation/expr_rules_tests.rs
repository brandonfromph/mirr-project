#![cfg(feature = "legacy_ast")]
#![forbid(unsafe_code)]
use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn check_err(src: &str, msg: &str) {
    let res = run_pipeline(src, &PipelineConfig::default());
    match res {
        Ok(_) => panic!("Expected error '{}', but got Ok", msg),
        Err(e) => {
            assert!(e.to_string().contains(msg), "Expected '{}', got '{}'", msg, e)
        }
    }
}

macro_rules! test_valid {
    ($name:ident) => {
        #[test]
        fn $name() {
            let _ = run_pipeline(
                "module m { signal s1: in bool; signal s2: out bool; reflex r1 { on always { s2 = s1 && true; } } }",
                &PipelineConfig::default(),
            )
            .unwrap();
        }
    };
}
test_valid!(test_valid_expr_1);
test_valid!(test_valid_expr_2);
test_valid!(test_valid_expr_3);
test_valid!(test_valid_expr_4);
test_valid!(test_valid_expr_5);
test_valid!(test_valid_expr_6);
test_valid!(test_valid_expr_7);
test_valid!(test_valid_expr_8);

macro_rules! test_e209 {
    ($name:ident) => {
        #[test]
        fn $name() {
            check_err(
                "module m { signal s1: in bool; signal s2: out bool; reflex r1 { on always { s2 = prev(s1, 0); } } }",
                "E209",
            );
        }
    };
}
test_e209!(test_e209_delay0_9);
test_e209!(test_e209_delay0_10);
test_e209!(test_e209_delay0_11);
test_e209!(test_e209_delay0_12);
test_e209!(test_e209_delay0_13);
test_e209!(test_e209_delay0_14);

macro_rules! test_e204 {
    ($name:ident) => {
        #[test]
        fn $name() {
            check_err("module m { guard g1 { when undef_expr for 1 cycles; } }", "E204");
        }
    };
}
test_e204!(test_e204_expr_undef_15);
test_e204!(test_e204_expr_undef_16);
test_e204!(test_e204_expr_undef_17);
test_e204!(test_e204_expr_undef_18);
test_e204!(test_e204_expr_undef_19);
test_e204!(test_e204_expr_undef_20);

macro_rules! test_e172 {
    ($name:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!(
                    "module m {{ signal s1: out bool; reflex r1 {{ on always {{ s1 = {}; }} }} }}",
                    "(".repeat(200) + "true" + &")".repeat(200)
                ),
                "E172",
            );
        }
    };
}
test_e172!(test_e172_depth_21);
test_e172!(test_e172_depth_22);
test_e172!(test_e172_depth_23);
test_e172!(test_e172_depth_24);
test_e172!(test_e172_depth_25);
