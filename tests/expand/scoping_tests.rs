#![forbid(unsafe_code)]
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
fn run_expand_only(src: &str) -> Result<(), String> {
    let config = PipelineConfig {
        typecheck: false,
        bootstrap_mode: false,
        simplify: false,
        sat_simplify: false,
        width: false,
        temporal: false,
        ..PipelineConfig::default()
    };
    run_pipeline(src, &config).map_err(|e| e.to_string()).map(|_| ())
}

macro_rules! test_scoping_ok {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            let src = &format!(
                "def A() {{ reflect {{ signal s: internal bool; }} }} module {} {{ A(); }}",
                stringify!($m)
            );
            let res = run_expand_only(src);
            assert!(res.is_ok(), "{:?}", res.err());
        }
    };
}
test_scoping_ok!(test_scoping_0, m_0);
test_scoping_ok!(test_scoping_1, m_1);
test_scoping_ok!(test_scoping_2, m_2);
test_scoping_ok!(test_scoping_3, m_3);
test_scoping_ok!(test_scoping_4, m_4);
test_scoping_ok!(test_scoping_5, m_5);
test_scoping_ok!(test_scoping_6, m_6);
test_scoping_ok!(test_scoping_7, m_7);
test_scoping_ok!(test_scoping_8, m_8);
test_scoping_ok!(test_scoping_9, m_9);

macro_rules! test_scoping_empty_ok {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            let src = &format!("module {} {{ }}", stringify!($m));
            let res = run_expand_only(src);
            assert!(res.is_ok(), "{:?}", res.err());
        }
    };
}
test_scoping_empty_ok!(test_scoping_10, m_10);
test_scoping_empty_ok!(test_scoping_11, m_11);
test_scoping_empty_ok!(test_scoping_12, m_12);
test_scoping_empty_ok!(test_scoping_13, m_13);
test_scoping_empty_ok!(test_scoping_14, m_14);

macro_rules! test_scoping_err {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            let src = &format!("def A(s: signal in bool) {{ reflect {{ A(s); }} }} module {} {{ signal sig: in u32; A(sig); }}", stringify!($m));
            let res = run_expand_only(src);
            assert!(res.is_err(), "{:?}", res.err());
        }
    };
}
test_scoping_err!(test_scoping_15, m_15);
test_scoping_err!(test_scoping_16, m_16);
test_scoping_err!(test_scoping_17, m_17);
test_scoping_err!(test_scoping_18, m_18);
test_scoping_err!(test_scoping_19, m_19);

macro_rules! test_scoping_nested_ok {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            let src = &format!("def A() {{ reflect {{ signal s: internal bool; }} }} def B() {{ reflect {{ A(); }} }} module {} {{ B(); }}", stringify!($m));
            let res = run_expand_only(src);
            assert!(res.is_ok(), "{:?}", res.err());
        }
    };
}
test_scoping_nested_ok!(test_scoping_20, m_20);
test_scoping_nested_ok!(test_scoping_21, m_21);
test_scoping_nested_ok!(test_scoping_22, m_22);
test_scoping_nested_ok!(test_scoping_23, m_23);
test_scoping_nested_ok!(test_scoping_24, m_24);
