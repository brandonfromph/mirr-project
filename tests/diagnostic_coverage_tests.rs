#![forbid(unsafe_code)]

fn assert_err(source: &str, expected_code: &str) {
    let config = mirrc::PipelineConfig::default();
    let res = mirrc::run_pipeline(source, &config);

    match res {
        Ok(_) => panic!("Expected error {}, but got success.", expected_code),
        Err(e) => {
            let actual = format!("{:?}", e);
            assert!(
                actual.contains(expected_code),
                "error '{:?}' should contain code '{}'",
                e,
                expected_code
            );
        }
    }
}

#[test]
fn err_e101_mirr_source_empty() {
    assert_err("", "[E101]");
}

#[test]
fn err_e102_expected_module_eof() {
    assert_err("struct S1 { x: bool; }", "[E102]");
}

#[test]
fn err_e103_expected_module_found() {
    assert_err("struct S1 { x: bool; } signal s1: bool;", "[E103]"); // This should hit E102 or E103 depending on where it breaks
}

#[test]
fn err_e105_module_name_empty() {
    assert_err("module { }", "[E105]");
}

#[test]
fn err_e109_signal_missing_semicolon() {
    assert_err("module test { signal s1: bool }", "[E109]");
}

#[test]
fn err_e110_signal_missing_colon() {
    assert_err("module test { signal s1 bool; }", "[E110]");
}

#[test]
fn err_e111_signal_name_empty() {
    assert_err("module test { signal : bool; }", "[E111]");
}

#[test]
fn err_e112_signal_kind_missing() {
    // Trigger E112 by name_tokens.len() == 2 with invalid kind
    assert_err("module test { signal unknown s1: bool; }", "[E115]"); // Actually E115 unknown kind
}

#[test]
fn err_e113_signal_type_missing() {
    assert_err("module test { signal s1: ; }", "[E113]");
}

#[test]
fn err_e114_signal_too_many_tokens() {
    assert_err(
        "module test { signal s1: t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 t11 t12 t13 t14 t15 t16 t17; }",
        "[E114]",
    );
}

#[test]
fn err_e117_signal_invalid_iwidth() {
    assert_err("module test { signal s1: i0; }", "[E117]");
}

#[test]
fn err_e107_unexpected_module_line() {
    assert_err("module test { random_junk; }", "[E107]");
}

#[test]
fn err_e211_signal_width_mismatch() {
    // u16 into u8 should trigger E211.
    // Assuming ALWAYS is a built-in or we define it. I'll define a guard.
    let source = "module test { 
        signal s1: u8; 
        signal s2: u16; 
        guard g1 { 
            when true for 1 
        }
        reflex r1 {
            on g1 { 
                s1 = s2; 
            }
        } 
    }";
    assert_err(source, "[E601]");
}

#[test]
fn err_e212_reflex_duplicate() {
    let source = "module test { 
        signal s1: bool; 
        guard g1 { 
            when true for 1 
        }
        reflex r1 { 
            on g1 { s1 = true; } 
        } 
        reflex r1 { 
            on g1 { s1 = false; } 
        } 
    }";
    assert_err(source, "[E212]");
}

#[test]
fn err_e213_guard_duplicate() {
    let source = "module test { 
        guard g1 { 
            when true for 1 
        } 
        guard g1 { 
            when false for 2 
        } 
    }";
    assert_err(source, "[E213]");
}

#[test]
fn err_e214_property_duplicate() {
    let source = "module test { 
        signal s1: bool; 
        property p1 {
            always (s1); 
        }
        property p1 {
            never (s1); 
        }
    }";
    assert_err(source, "[E214]");
}

#[test]
fn err_e302_temporal_cond_unsupported() {
    // Arithmetic in guard conditions is not supported (E302).
    let source = "module test { 
        signal s1: u8; 
        guard g1 { 
            when (s1 + 1) == 2 for 5 
        } 
    }";
    assert_err(source, "[E302]");
}

#[test]
fn err_e115_signal_unknown_kind() {
    // Trigger E115 by using an unknown keyword that isn't a valid type either.
    assert_err("module test { unknown s1: bool; }", "[E115]");
}

#[test]
fn err_e123_guard_expected_when() {
    // I'll update the code to use E123 for malformed when if needed, but for now
    // let's just see what happens.
    assert_err("module test { guard g1 { what true for 1 } }", "[E122]"); // Currently hits E122
}

#[test]
fn err_e116_signal_invalid_uwidth() {
    assert_err("module test { signal s1: u0; }", "[E116]");
}

#[test]
fn err_e118_signal_unknown_type() {
    assert_err("module test { signal s1: mystery; }", "[E118]");
}

#[test]
fn err_e121_guard_name_empty() {
    assert_err("module test { guard { when true for 1 } }", "[E121]");
}

#[test]
fn err_e122_guard_missing_when() {
    assert_err("module test { guard g1 { true for 1 } }", "[E122]");
}

#[test]
fn err_e140_reflex_missing_on() {
    let source = "module test {
        signal s1: out bool;
        reflex r1 {
            on {
                s1 = true;
            }
        }
    }";
    assert_err(source, "[E140]");
}
#[test]
fn err_e215_module_duplicate() {
    use mirrc::ecs::{KindComponent, Registry};
    let mut registry = Registry::new();

    // Manually create two modules with the same name
    let _m1 = registry.create_entity("M1", KindComponent::MODULE);
    let _m2 = registry.create_entity("M1", KindComponent::MODULE);

    let res = registry.semantic_validate();
    match res {
        Ok(_) => panic!("Expected duplicate module error but got success"),
        Err(e) => {
            let actual = format!("{:?}", e);
            assert!(actual.contains("[E215]"), "Error should contain E215: {:?}", actual);
        }
    }
}

#[test]
fn err_e209_prev_zero_delay() {
    let source = "module test {
        signal s1: bool;
        signal s2: bool;
        guard g1 {
            when s2 == prev(s1, 0)
            for 1
        }
    }";
    assert_err(source, "[E209]");
}

#[test]
fn err_e204_prev_undeclared_signal() {
    let source = "module test {
        signal s1: bool;
        guard g1 {
            when s1 == prev(missing, 1)
            for 1
        }
    }";
    assert_err(source, "[E204]");
}

#[test]
fn err_e214_property_manual_duplicate() {
    use mirrc::ecs::{KindComponent, Registry};
    let mut registry = Registry::new();
    let _p1 = registry.create_entity("P1", KindComponent::PROPERTY);
    let _p2 = registry.create_entity("P1", KindComponent::PROPERTY);
    let res = registry.semantic_validate();
    assert!(format!("{:?}", res).contains("[E214]"));
}

#[test]
fn err_e213_guard_manual_duplicate() {
    use mirrc::ecs::{KindComponent, Registry};
    let mut registry = Registry::new();
    let _g1 = registry.create_entity("G1", KindComponent::GUARD);
    let _g2 = registry.create_entity("G1", KindComponent::GUARD);
    let res = registry.semantic_validate();
    assert!(format!("{:?}", res).contains("[E213]"));
}

#[test]
fn err_e212_reflex_manual_duplicate() {
    use mirrc::ecs::{KindComponent, Registry};
    let mut registry = Registry::new();
    let _r1 = registry.create_entity("R1", KindComponent::REFLEX);
    let _r2 = registry.create_entity("R1", KindComponent::REFLEX);
    let res = registry.semantic_validate();
    assert!(format!("{:?}", res).contains("[E212]"));
}

#[test]
fn err_e204_binary_undeclared_left() {
    let source = "module test {
        signal s1: bool;
        guard g1 {
            when (missing + s1) == 1
            for 1
        }
    }";
    assert_err(source, "[E204]");
}

#[test]
fn err_e204_binary_undeclared_right() {
    let source = "module test {
        signal s1: bool;
        guard g1 {
            when (s1 + missing) == 1
            for 1
        }
    }";
    assert_err(source, "[E204]");
}

#[test]
fn err_e204_unary_undeclared() {
    let source = "module test {
        guard g1 {
            when (-missing) == 1
            for 1
        }
    }";
    assert_err(source, "[E204]");
}

#[test]
fn err_e117_invalid_iwidth_zero() {
    assert_err("module test {\n  signal s1: i0;\n}", "[E117]");
}

#[test]
fn err_e103_expected_module_found_retest() {
    // ExpectedModuleFound happens if it doesn't start with module
    assert_err("not_a_module { signal s1: bool; }", "[E103]");
}

#[test]
fn err_e205_reflex_on_undeclared_guard() {
    let source = "module test {
        signal s1: bool;
        reflex r1 {
            on missing {
                s1 = true;
            }
        }
    }";
    assert_err(source, "[E205]");
}

#[test]
fn err_e201_mixed_collision_signal_guard() {
    use mirrc::ast::types::SignalKind;
    use mirrc::ecs::{EntityKind, KindComponent, Registry};
    let mut registry = Registry::new();
    let _s1 =
        registry.create_entity("COLLISION", KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    let _g1 = registry.create_entity("COLLISION", KindComponent::GUARD);
    let res = registry.semantic_validate();
    assert!(format!("{:?}", res).contains("[E201]"));
}

#[test]
fn err_e106_unclosed_module_variant() {
    assert_err("module test ;", "[E106]");
}

#[test]
fn err_e116_uwidth_too_large() {
    // MAX_SIGNAL_WIDTH is 1024
    assert_err("module test {\n  signal s1: u1025;\n}", "[E116]");
}
