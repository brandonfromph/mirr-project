#![cfg(any())]
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

macro_rules! test_e204 {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!(
                    "module {} {{ guard g1 {{ when undef_sig for 1 cycles; }} }}",
                    stringify!($m)
                ),
                "E204",
            );
        }
    };
}
test_e204!(test_e204_guard_undef_1, m_1);
test_e204!(test_e204_guard_undef_2, m_2);
test_e204!(test_e204_guard_undef_3, m_3);
test_e204!(test_e204_guard_undef_4, m_4);
test_e204!(test_e204_guard_undef_5, m_5);
test_e204!(test_e204_guard_undef_6, m_6);

macro_rules! test_e205 {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!("module {} {{ signal s1: out bool; reflex r1 {{ on undef_guard {{ s1 = true; }} }} }}", stringify!($m)),
                "E205",
            );
        }
    };
}
test_e205!(test_e205_reflex_undef_7, m_7);
test_e205!(test_e205_reflex_undef_8, m_8);
test_e205!(test_e205_reflex_undef_9, m_9);
test_e205!(test_e205_reflex_undef_10, m_10);
test_e205!(test_e205_reflex_undef_11, m_11);
test_e205!(test_e205_reflex_undef_12, m_12);

macro_rules! test_e212 {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!("module {} {{ signal s1: out bool; reflex r1 {{ on always {{ s1 = true; }} }} reflex r1 {{ on always {{ s1 = false; }} }} }}", stringify!($m)),
                "E212",
            );
        }
    };
}
test_e212!(test_e212_duplicate_reflex_13, m_13);
test_e212!(test_e212_duplicate_reflex_14, m_14);
test_e212!(test_e212_duplicate_reflex_15, m_15);
test_e212!(test_e212_duplicate_reflex_16, m_16);
test_e212!(test_e212_duplicate_reflex_17, m_17);
test_e212!(test_e212_duplicate_reflex_18, m_18);
test_e212!(test_e212_duplicate_reflex_19, m_19);

macro_rules! test_e213 {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!("module {} {{ guard g1 {{ when true for 1 cycles; }} guard g1 {{ when false for 1 cycles; }} }}", stringify!($m)),
                "E213",
            );
        }
    };
}
test_e213!(test_e213_duplicate_guard_20, m_20);
test_e213!(test_e213_duplicate_guard_21, m_21);
test_e213!(test_e213_duplicate_guard_22, m_22);
test_e213!(test_e213_duplicate_guard_23, m_23);
test_e213!(test_e213_duplicate_guard_24, m_24);
test_e213!(test_e213_duplicate_guard_25, m_25);
