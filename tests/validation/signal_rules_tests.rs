#![forbid(unsafe_code)]
use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

fn check_ok(src: &str) {
    let p = parse_mirr(src).expect("Parse failed");
    validate_module(&p.module).expect("Validation failed");
}

fn check_err(src: &str, msg: &str) {
    let p = parse_mirr(src).expect("Parse failed, expected semantic error");
    match validate_module(&p.module) {
        Ok(_) => panic!("Expected error '{}', but got Ok", msg),
        Err(e) => {
            assert!(e.to_string().contains(msg), "Expected '{}', got '{}'", msg, e)
        }
    }
}

macro_rules! test_valid_sig {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_ok(&format!("module {} {{ signal s1: in bool; signal s2: out u8; reflex r1 {{ on always {{ s2 = s1; }} }} }}", stringify!($m)));
        }
    };
}
test_valid_sig!(test_valid_sig_1, m_1);
test_valid_sig!(test_valid_sig_2, m_2);
test_valid_sig!(test_valid_sig_3, m_3);
test_valid_sig!(test_valid_sig_4, m_4);
test_valid_sig!(test_valid_sig_5, m_5);

macro_rules! test_duplicate_sig {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!(
                    "module {} {{ signal s1: in bool; signal s1: out bool; }}",
                    stringify!($m)
                ),
                "E201",
            );
        }
    };
}
test_duplicate_sig!(test_duplicate_sig_6, m_6);
test_duplicate_sig!(test_duplicate_sig_7, m_7);
test_duplicate_sig!(test_duplicate_sig_8, m_8);
test_duplicate_sig!(test_duplicate_sig_9, m_9);
test_duplicate_sig!(test_duplicate_sig_10, m_10);

macro_rules! test_assign_input {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(&format!("module {} {{ signal s1: in bool; reflex r1 {{ on always {{ s1 = true; }} }} }}", stringify!($m)), "E206");
        }
    };
}
test_assign_input!(test_assign_input_11, m_11);
test_assign_input!(test_assign_input_12, m_12);
test_assign_input!(test_assign_input_13, m_13);
test_assign_input!(test_assign_input_14, m_14);
test_assign_input!(test_assign_input_15, m_15);

macro_rules! test_assign_undeclared {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(&format!("module {} {{ signal s1: in bool; reflex r1 {{ on always {{ s2 = true; }} }} }}", stringify!($m)), "E207");
        }
    };
}
test_assign_undeclared!(test_assign_undeclared_16, m_16);
test_assign_undeclared!(test_assign_undeclared_17, m_17);
test_assign_undeclared!(test_assign_undeclared_18, m_18);
test_assign_undeclared!(test_assign_undeclared_19, m_19);
test_assign_undeclared!(test_assign_undeclared_20, m_20);

macro_rules! test_multiple_writers {
    ($name:ident, $m:ident) => {
        #[test]
        fn $name() {
            check_err(
                &format!("module {} {{ signal s1: out bool; reflex r1 {{ on always {{ s1 = true; }} }} reflex r2 {{ on always {{ s1 = false; }} }} }}", stringify!($m)),
                "E216",
            );
        }
    };
}
test_multiple_writers!(test_multiple_writers_21, m_21);
test_multiple_writers!(test_multiple_writers_22, m_22);
test_multiple_writers!(test_multiple_writers_23, m_23);
test_multiple_writers!(test_multiple_writers_24, m_24);
test_multiple_writers!(test_multiple_writers_25, m_25);
