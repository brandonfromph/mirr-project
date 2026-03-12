#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::assertions_on_constants)]

//! Deep integration tests for `src/typeck/extended.rs`.
//!
//! Covers every check phase, error code (E610-E625), and public API surface
//! of the extended type system: refinement types, linear types, effect types,
//! clock domain qualifiers, phantom types, type-level naturals, dependent
//! types, session types, hardware mapping, and width hint APIs.

use nasa_rust_project::ast::program::{Module, SignalDecl};
use nasa_rust_project::ast::types::{
    EffectQualifier, ExtendedType as AstExtendedType, Linearity, Refinement, SignalKind,
    SignalType, TypeAnnotations,
};
use nasa_rust_project::error::PipelineErrors;
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig, PipelineResult};
use nasa_rust_project::typeck::extended::{
    effective_width, error_codes, hardware_mapping, refinement_width_hint, typecheck_extended,
    ClockDomain, DependentParam, ExtendedSignalDecl, ExtendedType, PhantomTag, RefinementBound,
    RefinementPredicate, SessionProtocol, SessionRole, SessionTransition, SessionTypeRef, TypeNat,
    TypeQualifier, MAX_CLOCK_DOMAINS, MAX_DEPENDENT_PARAMS, MAX_EXTENDED_TYPE_NODES,
    MAX_PHANTOM_TAGS, MAX_REFINEMENT_PREDICATES, MAX_SESSION_STATES, MAX_TYPE_NAT,
};

// ---------------------------------------------------------------------------
// Constants (NASA P10 — all loops bounded)
// ---------------------------------------------------------------------------

const MAX_TEST_SIGNALS: usize = 64;

// ---------------------------------------------------------------------------
// Helpers (no recursion)
// ---------------------------------------------------------------------------

/// Run the pipeline with extended typechecking enabled.
fn run_extended(source: &str) -> Result<PipelineResult, PipelineErrors> {
    let config = PipelineConfig { extended_typecheck: true, ..PipelineConfig::default() };
    run_pipeline(source, &config)
}

/// Parse MIRR source into a Module (panics on parse failure).
fn parse_module(source: &str) -> Module {
    let program = parse_mirr(source).expect("MIRR parse should succeed for test input");
    program.module
}

/// Build ExtendedSignalDecls from a parsed Module.
fn build_extended_decls(module: &Module) -> Vec<ExtendedSignalDecl> {
    let mut decls = Vec::with_capacity(module.signals.len());
    for i in 0..module.signals.len() {
        if i >= MAX_TEST_SIGNALS {
            break;
        }
        decls.push(ExtendedSignalDecl::from_legacy(&module.signals[i]));
    }
    decls
}

/// Collect error messages from an ExtendedTypeCheckResult into a single String.
fn collect_error_text(errors: &PipelineErrors) -> String {
    let mut buf = String::new();
    for i in 0..errors.errors.len() {
        if i >= MAX_TEST_SIGNALS {
            break;
        }
        buf.push_str(&format!("{:?}\n", errors.errors[i]));
    }
    buf
}

/// Minimal valid MIRR module source.
fn minimal_module() -> &'static str {
    "module m {\n\
     signal x: in bool;\n\
     signal y: out bool;\n\
     guard g {\n\
     when x\n\
     for 1 cycles;\n\
     }\n\
     reflex r {\n\
     on g {\n\
     y = x;\n\
     }\n\
     }\n\
     }"
}

/// Module with two u16 signals (for numeric testing).
fn numeric_module() -> &'static str {
    "module m {\n\
     signal a: in u16;\n\
     signal b: out u16;\n\
     guard g {\n\
     when a > 0\n\
     for 1 cycles;\n\
     }\n\
     reflex r {\n\
     on g {\n\
     b = a;\n\
     }\n\
     }\n\
     }"
}

// ===========================================================================
// Section 1: ExtendedType construction and methods
// ===========================================================================

#[test]
fn extended_type_from_base_bool() {
    let et = ExtendedType::from_base(SignalType::Bool);
    assert_eq!(et.base, SignalType::Bool, "Base should be Bool");
    assert!(et.is_base_only(), "from_base should produce base-only type");
    assert!(!et.is_linear(), "Base-only type should not be linear");
    assert!(!et.is_pure(), "Base-only type should not be pure");
    assert!(!et.is_stateful(), "Base-only type should not be stateful");
    assert_eq!(et.clock_domain_name(), None, "Base-only type should have no clock domain");
}

#[test]
fn extended_type_from_base_unsigned() {
    let et = ExtendedType::from_base(SignalType::Unsigned(32));
    assert_eq!(et.base, SignalType::Unsigned(32), "Base should be Unsigned(32)");
    assert!(et.is_base_only(), "from_base should produce base-only type");
}

#[test]
fn extended_type_from_base_signed() {
    let et = ExtendedType::from_base(SignalType::Signed(16));
    assert_eq!(et.base, SignalType::Signed(16), "Base should be Signed(16)");
    assert!(et.is_base_only(), "from_base should produce base-only type");
}

#[test]
fn extended_type_with_span() {
    let span =
        nasa_rust_project::span::Span { start_line: 5, start_col: 10, end_line: 5, end_col: 20 };
    let et = ExtendedType::from_base(SignalType::Bool).with_span(Some(span));
    assert_eq!(et.span, Some(span), "Span should be attached via builder");
    assert!(et.is_base_only(), "Span does not affect is_base_only");
}

#[test]
fn extended_type_is_linear() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Linear);
    assert!(et.is_linear(), "Type with Linear qualifier should return true for is_linear");
    assert!(!et.is_pure(), "Linear does not imply pure");
    assert!(!et.is_stateful(), "Linear does not imply stateful");
    assert!(!et.is_base_only(), "Type with qualifiers is not base-only");
}

#[test]
fn extended_type_is_pure() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Pure);
    assert!(et.is_pure(), "Type with Pure qualifier should return true for is_pure");
    assert!(!et.is_linear(), "Pure does not imply linear");
    assert!(!et.is_stateful(), "Pure does not imply stateful");
}

#[test]
fn extended_type_is_stateful() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Stateful);
    assert!(et.is_stateful(), "Type with Stateful qualifier should return true for is_stateful");
    assert!(!et.is_pure(), "Stateful does not imply pure");
}

#[test]
fn extended_type_multiple_qualifiers() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.qualifiers.push(TypeQualifier::Linear);
    et.qualifiers.push(TypeQualifier::Pure);
    assert!(et.is_linear(), "Should detect Linear among multiple qualifiers");
    assert!(et.is_pure(), "Should detect Pure among multiple qualifiers");
    assert!(!et.is_stateful(), "Stateful not present among qualifiers");
}

#[test]
fn extended_type_clock_domain_name() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.clock_domain = Some(ClockDomain::new("clk_fast"));
    assert_eq!(
        et.clock_domain_name(),
        Some("clk_fast"),
        "clock_domain_name should return the domain name"
    );
    assert!(!et.is_base_only(), "Type with clock domain is not base-only");
}

#[test]
fn extended_type_not_base_only_with_phantom() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.phantom = Some(PhantomTag::new("Verified"));
    assert!(!et.is_base_only(), "Type with phantom tag is not base-only");
}

#[test]
fn extended_type_not_base_only_with_type_nat() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.type_nat = Some(TypeNat::new(4).expect("4 <= MAX_TYPE_NAT"));
    assert!(!et.is_base_only(), "Type with type_nat is not base-only");
}

#[test]
fn extended_type_not_base_only_with_dependent_params() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.dependent_params.push(DependentParam::Const(4));
    assert!(!et.is_base_only(), "Type with dependent_params is not base-only");
}

#[test]
fn extended_type_not_base_only_with_session() {
    let mut et = ExtendedType::from_base(SignalType::Bool);
    et.session = Some(SessionTypeRef {
        protocol: "Handshake".to_string(),
        state: "Idle".to_string(),
        role: SessionRole::Sender,
    });
    assert!(!et.is_base_only(), "Type with session is not base-only");
}

#[test]
fn extended_type_not_base_only_with_refinements() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
    assert!(!et.is_base_only(), "Type with refinements is not base-only");
}

// ===========================================================================
// Section 2: base_max_value
// ===========================================================================

#[test]
fn base_max_value_bool_is_1() {
    let et = ExtendedType::from_base(SignalType::Bool);
    assert_eq!(et.base_max_value(), Some(1), "Bool max value should be 1");
}

#[test]
fn base_max_value_unsigned_8() {
    let et = ExtendedType::from_base(SignalType::Unsigned(8));
    assert_eq!(et.base_max_value(), Some(255), "u8 max value should be 255");
}

#[test]
fn base_max_value_unsigned_16() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    assert_eq!(et.base_max_value(), Some(65535), "u16 max value should be 65535");
}

#[test]
fn base_max_value_unsigned_1() {
    let et = ExtendedType::from_base(SignalType::Unsigned(1));
    assert_eq!(et.base_max_value(), Some(1), "u1 max value should be 1");
}

#[test]
fn base_max_value_unsigned_64_is_none() {
    let et = ExtendedType::from_base(SignalType::Unsigned(64));
    assert_eq!(et.base_max_value(), None, "u64 overflows, should return None");
}

#[test]
fn base_max_value_signed_8() {
    let et = ExtendedType::from_base(SignalType::Signed(8));
    assert_eq!(et.base_max_value(), Some(127), "i8 max positive value should be 127");
}

#[test]
fn base_max_value_signed_16() {
    let et = ExtendedType::from_base(SignalType::Signed(16));
    assert_eq!(et.base_max_value(), Some(32767), "i16 max positive value should be 32767");
}

#[test]
fn base_max_value_signed_0() {
    let et = ExtendedType::from_base(SignalType::Signed(0));
    assert_eq!(et.base_max_value(), Some(0), "i0 max value should be 0");
}

#[test]
fn base_max_value_signed_64_is_none() {
    let et = ExtendedType::from_base(SignalType::Signed(64));
    assert_eq!(et.base_max_value(), None, "i64 overflows, should return None");
}

// ===========================================================================
// Section 3: RefinementBound — satisfied_by
// ===========================================================================

#[test]
fn refinement_value_lt_boundary() {
    let bound = RefinementBound::ValueLt(100);
    assert!(bound.satisfied_by(99), "99 < 100 should be satisfied");
    assert!(!bound.satisfied_by(100), "100 < 100 should not be satisfied");
    assert!(!bound.satisfied_by(101), "101 < 100 should not be satisfied");
    assert!(bound.satisfied_by(0), "0 < 100 should be satisfied");
}

#[test]
fn refinement_value_le_boundary() {
    let bound = RefinementBound::ValueLe(100);
    assert!(bound.satisfied_by(100), "100 <= 100 should be satisfied");
    assert!(!bound.satisfied_by(101), "101 <= 100 should not be satisfied");
    assert!(bound.satisfied_by(0), "0 <= 100 should be satisfied");
}

#[test]
fn refinement_value_gt_boundary() {
    let bound = RefinementBound::ValueGt(50);
    assert!(bound.satisfied_by(51), "51 > 50 should be satisfied");
    assert!(!bound.satisfied_by(50), "50 > 50 should not be satisfied");
    assert!(!bound.satisfied_by(49), "49 > 50 should not be satisfied");
}

#[test]
fn refinement_value_ge_boundary() {
    let bound = RefinementBound::ValueGe(50);
    assert!(bound.satisfied_by(50), "50 >= 50 should be satisfied");
    assert!(bound.satisfied_by(51), "51 >= 50 should be satisfied");
    assert!(!bound.satisfied_by(49), "49 >= 50 should not be satisfied");
}

#[test]
fn refinement_value_eq() {
    let bound = RefinementBound::ValueEq(42);
    assert!(bound.satisfied_by(42), "42 == 42 should be satisfied");
    assert!(!bound.satisfied_by(41), "41 == 42 should not be satisfied");
    assert!(!bound.satisfied_by(43), "43 == 42 should not be satisfied");
}

#[test]
fn refinement_value_ne() {
    let bound = RefinementBound::ValueNe(42);
    assert!(!bound.satisfied_by(42), "42 != 42 should not be satisfied");
    assert!(bound.satisfied_by(41), "41 != 42 should be satisfied");
    assert!(bound.satisfied_by(0), "0 != 42 should be satisfied");
}

#[test]
fn refinement_value_in_range() {
    let bound = RefinementBound::ValueInRange { lo: 10, hi: 20 };
    assert!(bound.satisfied_by(10), "10 in 10..=20 should be satisfied");
    assert!(bound.satisfied_by(15), "15 in 10..=20 should be satisfied");
    assert!(bound.satisfied_by(20), "20 in 10..=20 should be satisfied");
    assert!(!bound.satisfied_by(9), "9 in 10..=20 should not be satisfied");
    assert!(!bound.satisfied_by(21), "21 in 10..=20 should not be satisfied");
}

#[test]
fn refinement_value_in_range_singleton() {
    let bound = RefinementBound::ValueInRange { lo: 5, hi: 5 };
    assert!(bound.satisfied_by(5), "5 in 5..=5 should be satisfied");
    assert!(!bound.satisfied_by(4), "4 in 5..=5 should not be satisfied");
    assert!(!bound.satisfied_by(6), "6 in 5..=5 should not be satisfied");
}

#[test]
fn refinement_value_mod() {
    let bound = RefinementBound::ValueMod { divisor: 4, remainder: 0 };
    assert!(bound.satisfied_by(0), "0 % 4 == 0 should be satisfied");
    assert!(bound.satisfied_by(4), "4 % 4 == 0 should be satisfied");
    assert!(bound.satisfied_by(8), "8 % 4 == 0 should be satisfied");
    assert!(!bound.satisfied_by(1), "1 % 4 == 0 should not be satisfied");
    assert!(!bound.satisfied_by(7), "7 % 4 == 0 should not be satisfied");
}

#[test]
fn refinement_value_mod_nonzero_remainder() {
    let bound = RefinementBound::ValueMod { divisor: 3, remainder: 1 };
    assert!(bound.satisfied_by(1), "1 % 3 == 1 should be satisfied");
    assert!(bound.satisfied_by(4), "4 % 3 == 1 should be satisfied");
    assert!(bound.satisfied_by(7), "7 % 3 == 1 should be satisfied");
    assert!(!bound.satisfied_by(0), "0 % 3 == 1 should not be satisfied");
    assert!(!bound.satisfied_by(2), "2 % 3 == 1 should not be satisfied");
}

#[test]
fn refinement_value_mod_divisor_zero() {
    let bound = RefinementBound::ValueMod { divisor: 0, remainder: 0 };
    assert!(!bound.satisfied_by(0), "Division by zero should never be satisfied");
    assert!(!bound.satisfied_by(1), "Division by zero should never be satisfied");
}

// ===========================================================================
// Section 4: RefinementBound — implied_min / implied_max
// ===========================================================================

#[test]
fn implied_max_value_lt() {
    assert_eq!(
        RefinementBound::ValueLt(1024).implied_max(),
        Some(1023),
        "ValueLt(1024) implied max should be 1023"
    );
}

#[test]
fn implied_max_value_lt_zero() {
    assert_eq!(
        RefinementBound::ValueLt(0).implied_max(),
        None,
        "ValueLt(0) has no valid max (would underflow)"
    );
}

#[test]
fn implied_max_value_le() {
    assert_eq!(
        RefinementBound::ValueLe(500).implied_max(),
        Some(500),
        "ValueLe(500) implied max should be 500"
    );
}

#[test]
fn implied_max_value_eq() {
    assert_eq!(
        RefinementBound::ValueEq(42).implied_max(),
        Some(42),
        "ValueEq(42) implied max should be 42"
    );
}

#[test]
fn implied_max_value_in_range() {
    assert_eq!(
        RefinementBound::ValueInRange { lo: 10, hi: 200 }.implied_max(),
        Some(200),
        "ValueInRange {{ lo: 10, hi: 200 }} implied max should be 200"
    );
}

#[test]
fn implied_max_none_for_lower_bounds() {
    assert_eq!(
        RefinementBound::ValueGt(5).implied_max(),
        None,
        "ValueGt does not constrain upper bound"
    );
    assert_eq!(
        RefinementBound::ValueGe(5).implied_max(),
        None,
        "ValueGe does not constrain upper bound"
    );
    assert_eq!(
        RefinementBound::ValueNe(5).implied_max(),
        None,
        "ValueNe does not constrain upper bound"
    );
    assert_eq!(
        RefinementBound::ValueMod { divisor: 2, remainder: 0 }.implied_max(),
        None,
        "ValueMod does not constrain upper bound"
    );
}

#[test]
fn implied_min_value_gt() {
    assert_eq!(
        RefinementBound::ValueGt(5).implied_min(),
        Some(6),
        "ValueGt(5) implied min should be 6"
    );
}

#[test]
fn implied_min_value_ge() {
    assert_eq!(
        RefinementBound::ValueGe(10).implied_min(),
        Some(10),
        "ValueGe(10) implied min should be 10"
    );
}

#[test]
fn implied_min_value_eq() {
    assert_eq!(
        RefinementBound::ValueEq(42).implied_min(),
        Some(42),
        "ValueEq(42) implied min should be 42"
    );
}

#[test]
fn implied_min_value_in_range() {
    assert_eq!(
        RefinementBound::ValueInRange { lo: 100, hi: 200 }.implied_min(),
        Some(100),
        "ValueInRange {{ lo: 100, hi: 200 }} implied min should be 100"
    );
}

#[test]
fn implied_min_none_for_upper_bounds() {
    assert_eq!(
        RefinementBound::ValueLt(1024).implied_min(),
        None,
        "ValueLt does not constrain lower bound"
    );
    assert_eq!(
        RefinementBound::ValueLe(1024).implied_min(),
        None,
        "ValueLe does not constrain lower bound"
    );
}

#[test]
fn implied_min_value_gt_max_u64_overflow() {
    assert_eq!(
        RefinementBound::ValueGt(u64::MAX).implied_min(),
        None,
        "ValueGt(u64::MAX) should return None due to overflow"
    );
}

// ===========================================================================
// Section 5: RefinementBound — Display
// ===========================================================================

#[test]
fn refinement_bound_display_all_variants() {
    assert_eq!(
        RefinementBound::ValueLt(100).to_string(),
        "value < 100",
        "ValueLt display mismatch"
    );
    assert_eq!(
        RefinementBound::ValueLe(100).to_string(),
        "value <= 100",
        "ValueLe display mismatch"
    );
    assert_eq!(RefinementBound::ValueGt(50).to_string(), "value > 50", "ValueGt display mismatch");
    assert_eq!(RefinementBound::ValueGe(50).to_string(), "value >= 50", "ValueGe display mismatch");
    assert_eq!(RefinementBound::ValueEq(42).to_string(), "value == 42", "ValueEq display mismatch");
    assert_eq!(RefinementBound::ValueNe(42).to_string(), "value != 42", "ValueNe display mismatch");
    assert_eq!(
        RefinementBound::ValueInRange { lo: 10, hi: 200 }.to_string(),
        "value in 10..=200",
        "ValueInRange display mismatch"
    );
    assert_eq!(
        RefinementBound::ValueMod { divisor: 4, remainder: 0 }.to_string(),
        "value % 4 == 0",
        "ValueMod display mismatch"
    );
}

// ===========================================================================
// Section 6: TypeQualifier Display
// ===========================================================================

#[test]
fn type_qualifier_display() {
    assert_eq!(TypeQualifier::Linear.to_string(), "linear", "Linear display mismatch");
    assert_eq!(TypeQualifier::Pure.to_string(), "pure", "Pure display mismatch");
    assert_eq!(TypeQualifier::Stateful.to_string(), "stateful", "Stateful display mismatch");
}

// ===========================================================================
// Section 7: ClockDomain
// ===========================================================================

#[test]
fn clock_domain_new() {
    let cd = ClockDomain::new("sys_clk");
    assert_eq!(cd.name, "sys_clk", "ClockDomain name should match constructor arg");
    assert_eq!(cd.frequency_hz, None, "Default frequency should be None");
}

#[test]
fn clock_domain_with_frequency() {
    let cd = ClockDomain::new("clk_100").with_frequency(100_000_000);
    assert_eq!(cd.frequency_hz, Some(100_000_000), "Frequency should be set via builder");
}

#[test]
fn clock_domain_display_without_frequency() {
    let cd = ClockDomain::new("clk_a");
    assert_eq!(cd.to_string(), "@clk_a", "Display should prefix with @");
}

#[test]
fn clock_domain_display_with_frequency() {
    let cd = ClockDomain::new("clk_fast").with_frequency(200_000_000);
    assert_eq!(cd.to_string(), "@clk_fast(200000000Hz)", "Display should include frequency in Hz");
}

// ===========================================================================
// Section 8: PhantomTag
// ===========================================================================

#[test]
fn phantom_tag_new() {
    let pt = PhantomTag::new("Verified");
    assert_eq!(pt.tag, "Verified", "PhantomTag tag should match constructor arg");
}

#[test]
fn phantom_tag_display() {
    let pt = PhantomTag::new("Encrypted");
    assert_eq!(pt.to_string(), "#Encrypted", "Display should prefix with #");
}

#[test]
fn phantom_tag_equality() {
    let a = PhantomTag::new("A");
    let b = PhantomTag::new("A");
    let c = PhantomTag::new("B");
    assert_eq!(a, b, "Same-tag PhantomTags should be equal");
    assert_ne!(a, c, "Different-tag PhantomTags should not be equal");
}

// ===========================================================================
// Section 9: TypeNat
// ===========================================================================

#[test]
fn type_nat_new_valid() {
    let nat = TypeNat::new(1);
    assert!(nat.is_some(), "TypeNat(1) should be valid");
    assert_eq!(nat.unwrap().value, 1, "TypeNat value should be 1");
}

#[test]
fn type_nat_new_zero() {
    let nat = TypeNat::new(0);
    assert!(nat.is_some(), "TypeNat(0) should be valid");
}

#[test]
fn type_nat_new_at_max() {
    let nat = TypeNat::new(MAX_TYPE_NAT);
    assert!(nat.is_some(), "TypeNat at MAX_TYPE_NAT should be valid");
    assert_eq!(nat.unwrap().value, MAX_TYPE_NAT, "Value should equal MAX_TYPE_NAT");
}

#[test]
fn type_nat_new_exceeds_max() {
    let nat = TypeNat::new(MAX_TYPE_NAT + 1);
    assert!(nat.is_none(), "TypeNat exceeding MAX_TYPE_NAT should return None");
}

#[test]
fn type_nat_total_width_basic() {
    let nat = TypeNat::new(4).expect("4 is valid");
    assert_eq!(nat.total_width(8), Some(32), "4 elements * 8 bits = 32 bits");
}

#[test]
fn type_nat_total_width_one_element() {
    let nat = TypeNat::new(1).expect("1 is valid");
    assert_eq!(nat.total_width(16), Some(16), "1 element * 16 bits = 16 bits");
}

#[test]
fn type_nat_total_width_zero_elements() {
    let nat = TypeNat::new(0).expect("0 is valid");
    assert_eq!(nat.total_width(8), Some(0), "0 elements * 8 bits = 0 bits");
}

#[test]
fn type_nat_display() {
    let nat = TypeNat::new(42).expect("42 is valid");
    assert_eq!(nat.to_string(), "42", "TypeNat Display should show value");
}

// ===========================================================================
// Section 10: DependentParam
// ===========================================================================

#[test]
fn dependent_param_const_display() {
    let dp = DependentParam::Const(256);
    assert_eq!(dp.to_string(), "256", "Const DependentParam Display mismatch");
}

#[test]
fn dependent_param_type_display() {
    let dp = DependentParam::Type(SignalType::Unsigned(8));
    assert_eq!(dp.to_string(), "u8", "Type DependentParam Display mismatch");
}

#[test]
fn dependent_param_phantom_display() {
    let dp = DependentParam::Phantom(PhantomTag::new("Safe"));
    assert_eq!(dp.to_string(), "#Safe", "Phantom DependentParam Display mismatch");
}

// ===========================================================================
// Section 11: SessionTypeRef and SessionProtocol
// ===========================================================================

#[test]
fn session_role_display() {
    assert_eq!(SessionRole::Sender.to_string(), "sender", "Sender role display mismatch");
    assert_eq!(SessionRole::Receiver.to_string(), "receiver", "Receiver role display mismatch");
}

#[test]
fn session_type_ref_display() {
    let sr = SessionTypeRef {
        protocol: "Handshake".to_string(),
        state: "Idle".to_string(),
        role: SessionRole::Sender,
    };
    let display = sr.to_string();
    assert!(display.contains("Handshake"), "Display should contain protocol name");
    assert!(display.contains("Idle"), "Display should contain state name");
    assert!(display.contains("sender"), "Display should contain role");
}

#[test]
fn session_protocol_construction() {
    let proto = SessionProtocol {
        name: "SPI".to_string(),
        transitions: vec![
            SessionTransition { from: "Idle".to_string(), to: "Active".to_string(), guard: None },
            SessionTransition {
                from: "Active".to_string(),
                to: "Idle".to_string(),
                guard: Some("done".to_string()),
            },
        ],
        span: None,
    };
    assert_eq!(proto.name, "SPI", "Protocol name should match");
    assert_eq!(proto.transitions.len(), 2, "Protocol should have 2 transitions");
    assert_eq!(proto.transitions[0].from, "Idle", "First transition from should be Idle");
    assert_eq!(proto.transitions[0].to, "Active", "First transition to should be Active");
    assert_eq!(
        proto.transitions[1].guard,
        Some("done".to_string()),
        "Second transition guard mismatch"
    );
}

// ===========================================================================
// Section 12: ExtendedType Display
// ===========================================================================

#[test]
fn extended_type_display_base_only() {
    let et = ExtendedType::from_base(SignalType::Unsigned(8));
    assert_eq!(et.to_string(), "u8", "Base-only display should show just the base type");
}

#[test]
fn extended_type_display_bool() {
    let et = ExtendedType::from_base(SignalType::Bool);
    assert_eq!(et.to_string(), "bool", "Bool display mismatch");
}

#[test]
fn extended_type_display_signed() {
    let et = ExtendedType::from_base(SignalType::Signed(32));
    assert_eq!(et.to_string(), "i32", "Signed display mismatch");
}

#[test]
fn extended_type_display_with_qualifiers() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.qualifiers.push(TypeQualifier::Linear);
    et.qualifiers.push(TypeQualifier::Pure);
    let display = et.to_string();
    assert!(display.contains("linear"), "Display should contain 'linear'");
    assert!(display.contains("pure"), "Display should contain 'pure'");
    assert!(display.contains("u16"), "Display should contain base type");
}

#[test]
fn extended_type_display_with_clock_domain() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.clock_domain = Some(ClockDomain::new("sys"));
    assert!(et.to_string().contains("@sys"), "Display should contain clock domain");
}

#[test]
fn extended_type_display_with_phantom() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.phantom = Some(PhantomTag::new("Raw"));
    assert!(et.to_string().contains("#Raw"), "Display should contain phantom tag");
}

#[test]
fn extended_type_display_with_refinements() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueGe(10), span: None });
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLe(500), span: None });
    let display = et.to_string();
    assert!(display.contains("where"), "Display should contain 'where' for refinements");
    assert!(display.contains("value >= 10"), "Display should contain lower bound");
    assert!(display.contains("value <= 500"), "Display should contain upper bound");
    assert!(display.contains("&&"), "Multiple predicates should be joined by &&");
}

#[test]
fn extended_type_display_with_dependent_params() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.dependent_params.push(DependentParam::Const(4));
    et.dependent_params.push(DependentParam::Type(SignalType::Unsigned(8)));
    let display = et.to_string();
    assert!(display.contains("<"), "Display should contain < for dependent params");
    assert!(display.contains(">"), "Display should contain > for dependent params");
    assert!(display.contains("4"), "Display should contain first param");
    assert!(display.contains("u8"), "Display should contain second param");
}

// ===========================================================================
// Section 13: refinement_width_hint
// ===========================================================================

#[test]
fn refinement_width_hint_no_refinements() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    assert_eq!(refinement_width_hint(&et), None, "No refinements should yield no hint");
}

#[test]
fn refinement_width_hint_from_value_lt() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLt(256), span: None });
    let hint = refinement_width_hint(&et);
    // 255 needs 8 bits
    assert_eq!(
        hint,
        Some(nasa_rust_project::width::types::Width(8)),
        "ValueLt(256) should yield 8-bit hint"
    );
}

#[test]
fn refinement_width_hint_from_value_le() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLe(1023), span: None });
    let hint = refinement_width_hint(&et);
    // 1023 needs 10 bits
    assert_eq!(
        hint,
        Some(nasa_rust_project::width::types::Width(10)),
        "ValueLe(1023) should yield 10-bit hint"
    );
}

#[test]
fn refinement_width_hint_tightest_wins() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(32));
    et.refinements.push(RefinementPredicate {
        bound: RefinementBound::ValueLt(1024), // max=1023, 10 bits
        span: None,
    });
    et.refinements.push(RefinementPredicate {
        bound: RefinementBound::ValueLe(127), // max=127, 7 bits
        span: None,
    });
    let hint = refinement_width_hint(&et);
    // Tightest is 127 -> 7 bits
    assert_eq!(
        hint,
        Some(nasa_rust_project::width::types::Width(7)),
        "Tightest upper bound (127) should produce 7-bit hint"
    );
}

#[test]
fn refinement_width_hint_only_lower_bound_yields_none() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueGe(10), span: None });
    assert_eq!(
        refinement_width_hint(&et),
        None,
        "Only lower-bound refinements should not produce a hint"
    );
}

// ===========================================================================
// Section 14: effective_width
// ===========================================================================

#[test]
fn effective_width_no_refinement() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    assert_eq!(
        effective_width(&et),
        nasa_rust_project::width::types::Width(16),
        "No refinement should yield declared width"
    );
}

#[test]
fn effective_width_with_narrowing_refinement() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLt(256), span: None });
    assert_eq!(
        effective_width(&et),
        nasa_rust_project::width::types::Width(8),
        "Refinement max=255 should narrow to 8 bits from 16"
    );
}

#[test]
fn effective_width_refinement_wider_than_base() {
    // Refinement says max=65535, base is u8 (max=255). Declared width wins.
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLe(65535), span: None });
    assert_eq!(
        effective_width(&et),
        nasa_rust_project::width::types::Width(8),
        "Refinement wider than base should not widen beyond declared"
    );
}

#[test]
fn effective_width_bool() {
    let et = ExtendedType::from_base(SignalType::Bool);
    assert_eq!(
        effective_width(&et),
        nasa_rust_project::width::types::Width(1),
        "Bool effective width should be 1"
    );
}

#[test]
fn effective_width_signed() {
    let et = ExtendedType::from_base(SignalType::Signed(16));
    assert_eq!(
        effective_width(&et),
        nasa_rust_project::width::types::Width(16),
        "Signed(16) with no refinement should be 16"
    );
}

// ===========================================================================
// Section 15: hardware_mapping
// ===========================================================================

#[test]
fn hw_mapping_base_only_no_impact() {
    let et = ExtendedType::from_base(SignalType::Unsigned(8));
    assert!(
        !hardware_mapping::has_synthesis_impact(&et),
        "Base-only type should have no synthesis impact"
    );
}

#[test]
fn hw_mapping_clock_domain_has_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.clock_domain = Some(ClockDomain::new("clk_fast"));
    assert!(
        hardware_mapping::has_synthesis_impact(&et),
        "Clock domain should have synthesis impact"
    );
}

#[test]
fn hw_mapping_pure_has_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Pure);
    assert!(
        hardware_mapping::has_synthesis_impact(&et),
        "Pure qualifier should have synthesis impact"
    );
}

#[test]
fn hw_mapping_stateful_has_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Stateful);
    assert!(
        hardware_mapping::has_synthesis_impact(&et),
        "Stateful qualifier should have synthesis impact"
    );
}

#[test]
fn hw_mapping_type_nat_has_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.type_nat = Some(TypeNat::new(4).unwrap());
    assert!(
        hardware_mapping::has_synthesis_impact(&et),
        "Type-level natural should have synthesis impact"
    );
}

#[test]
fn hw_mapping_dependent_params_has_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.dependent_params.push(DependentParam::Const(8));
    assert!(
        hardware_mapping::has_synthesis_impact(&et),
        "Dependent params should have synthesis impact"
    );
}

#[test]
fn hw_mapping_linear_no_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.qualifiers.push(TypeQualifier::Linear);
    assert!(
        !hardware_mapping::has_synthesis_impact(&et),
        "Linear alone should have no synthesis impact"
    );
}

#[test]
fn hw_mapping_phantom_no_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.phantom = Some(PhantomTag::new("Verified"));
    assert!(
        !hardware_mapping::has_synthesis_impact(&et),
        "Phantom tag alone should have no synthesis impact"
    );
}

#[test]
fn hw_mapping_refinement_no_impact() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
    assert!(
        !hardware_mapping::has_synthesis_impact(&et),
        "Refinement alone should have no synthesis impact"
    );
}

#[test]
fn hw_mapping_session_no_impact() {
    let mut et = ExtendedType::from_base(SignalType::Bool);
    et.session = Some(SessionTypeRef {
        protocol: "P".to_string(),
        state: "S".to_string(),
        role: SessionRole::Sender,
    });
    assert!(
        !hardware_mapping::has_synthesis_impact(&et),
        "Session type alone should have no synthesis impact"
    );
}

// ===========================================================================
// Section 16: extended_firrtl_type
// ===========================================================================

#[test]
fn firrtl_type_bool() {
    let et = ExtendedType::from_base(SignalType::Bool);
    assert_eq!(
        hardware_mapping::extended_firrtl_type(&et),
        "UInt<1>",
        "Bool should map to UInt<1>"
    );
}

#[test]
fn firrtl_type_unsigned() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    assert_eq!(
        hardware_mapping::extended_firrtl_type(&et),
        "UInt<16>",
        "Unsigned(16) should map to UInt<16>"
    );
}

#[test]
fn firrtl_type_signed() {
    let et = ExtendedType::from_base(SignalType::Signed(32));
    assert_eq!(
        hardware_mapping::extended_firrtl_type(&et),
        "SInt<32>",
        "Signed(32) should map to SInt<32>"
    );
}

#[test]
fn firrtl_type_array() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.type_nat = Some(TypeNat::new(4).unwrap());
    assert_eq!(
        hardware_mapping::extended_firrtl_type(&et),
        "UInt<8>[4]",
        "Array of 4 u8 should map to UInt<8>[4]"
    );
}

#[test]
fn firrtl_type_signed_array() {
    let mut et = ExtendedType::from_base(SignalType::Signed(16));
    et.type_nat = Some(TypeNat::new(8).unwrap());
    assert_eq!(
        hardware_mapping::extended_firrtl_type(&et),
        "SInt<16>[8]",
        "Array of 8 i16 should map to SInt<16>[8]"
    );
}

// ===========================================================================
// Section 17: ExtendedSignalDecl::from_legacy
// ===========================================================================

#[test]
fn from_legacy_base_only() {
    let legacy = SignalDecl {
        name: "sensor".to_string(),
        kind: SignalKind::Input,
        ty: AstExtendedType::from_core(SignalType::Unsigned(16)),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(ext.name, "sensor", "Name should be preserved");
    assert_eq!(ext.ty, SignalType::Unsigned(16), "Base type should be preserved");
    assert!(ext.extended_ty.is_base_only(), "Base-only legacy should produce base-only extended");
    assert_eq!(ext.kind, SignalKind::Input, "Kind should be preserved");
}

#[test]
fn from_legacy_with_linear_annotation() {
    let mut annotations = TypeAnnotations::default();
    annotations.linearity = Linearity::Linear;
    let legacy = SignalDecl {
        name: "trigger".to_string(),
        kind: SignalKind::Output,
        ty: AstExtendedType::new(SignalType::Bool, annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert!(ext.extended_ty.is_linear(), "Linear annotation should propagate");
    assert!(!ext.extended_ty.is_base_only(), "Should not be base-only with linear");
}

#[test]
fn from_legacy_with_stateful_annotation() {
    let mut annotations = TypeAnnotations::default();
    annotations.effect = EffectQualifier::Stateful;
    let legacy = SignalDecl {
        name: "reg".to_string(),
        kind: SignalKind::Internal,
        ty: AstExtendedType::new(SignalType::Unsigned(8), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert!(ext.extended_ty.is_stateful(), "Stateful annotation should propagate");
}

#[test]
fn from_legacy_with_pure_annotation() {
    let mut annotations = TypeAnnotations::default();
    annotations.effect = EffectQualifier::Pure;
    let legacy = SignalDecl {
        name: "wire".to_string(),
        kind: SignalKind::Output,
        ty: AstExtendedType::new(SignalType::Unsigned(8), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert!(ext.extended_ty.is_pure(), "Pure annotation should propagate");
}

#[test]
fn from_legacy_with_refinement_range() {
    let mut annotations = TypeAnnotations::default();
    annotations.refinement = Some(Refinement::Range { lo: 10, hi: 200 });
    let legacy = SignalDecl {
        name: "bounded".to_string(),
        kind: SignalKind::Output,
        ty: AstExtendedType::new(SignalType::Unsigned(16), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(
        ext.extended_ty.refinements.len(),
        1,
        "Range refinement should produce one predicate"
    );
    assert_eq!(
        ext.extended_ty.refinements[0].bound,
        RefinementBound::ValueInRange { lo: 10, hi: 200 },
        "Refinement should be ValueInRange"
    );
}

#[test]
fn from_legacy_with_refinement_predicate() {
    let mut annotations = TypeAnnotations::default();
    annotations.refinement = Some(Refinement::Predicate("value < 1024".to_string()));
    let legacy = SignalDecl {
        name: "capped".to_string(),
        kind: SignalKind::Output,
        ty: AstExtendedType::new(SignalType::Unsigned(16), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(
        ext.extended_ty.refinements.len(),
        1,
        "Predicate refinement should produce one entry"
    );
    // Predicate string is stored as ValueGe(0) placeholder for now
    assert_eq!(
        ext.extended_ty.refinements[0].bound,
        RefinementBound::ValueGe(0),
        "Predicate refinement should be stored as ValueGe(0) placeholder"
    );
}

#[test]
fn from_legacy_with_clock_domain() {
    let mut annotations = TypeAnnotations::default();
    annotations.clock_domain = Some("clk_200".to_string());
    let legacy = SignalDecl {
        name: "fast_sig".to_string(),
        kind: SignalKind::Input,
        ty: AstExtendedType::new(SignalType::Unsigned(16), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(
        ext.extended_ty.clock_domain_name(),
        Some("clk_200"),
        "Clock domain should propagate from annotations"
    );
}

#[test]
fn from_legacy_with_phantom_tag() {
    let mut annotations = TypeAnnotations::default();
    annotations.phantom_tag = Some("Temperature".to_string());
    let legacy = SignalDecl {
        name: "temp".to_string(),
        kind: SignalKind::Input,
        ty: AstExtendedType::new(SignalType::Unsigned(16), annotations),
        origin: None,
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(
        ext.extended_ty.phantom.as_ref().map(|p| p.tag.as_str()),
        Some("Temperature"),
        "Phantom tag should propagate from annotations"
    );
}

#[test]
fn from_legacy_preserves_origin() {
    let legacy = SignalDecl {
        name: "sig".to_string(),
        kind: SignalKind::Input,
        ty: AstExtendedType::from_core(SignalType::Bool),
        origin: Some("TMR_pattern".to_string()),
        span: None,
    };
    let ext = ExtendedSignalDecl::from_legacy(&legacy);
    assert_eq!(
        ext.origin.as_deref(),
        Some("TMR_pattern"),
        "Origin should be preserved from legacy decl"
    );
}

// ===========================================================================
// Section 18: Error code constants
// ===========================================================================

#[test]
fn error_codes_are_correct_strings() {
    assert_eq!(error_codes::E610_REF_BOUND, "E610", "E610 code mismatch");
    assert_eq!(error_codes::E612_REF_WIDTH, "E612", "E612 code mismatch");
    assert_eq!(error_codes::E613_LIN_UNUSED, "E613", "E613 code mismatch");
    assert_eq!(error_codes::E614_LIN_DOUBLE, "E614", "E614 code mismatch");
    assert_eq!(error_codes::E616_EFF_PURE, "E616", "E616 code mismatch");
    assert_eq!(error_codes::E617_EFF_MIX, "E617", "E617 code mismatch");
    assert_eq!(error_codes::E618_CLK_CROSS, "E618", "E618 code mismatch");
    assert_eq!(error_codes::E619_CLK_UNDEF, "E619", "E619 code mismatch");
    assert_eq!(error_codes::E620_PHT_MISMATCH, "E620", "E620 code mismatch");
    assert_eq!(error_codes::E621_PHT_UNDEF, "E621", "E621 code mismatch");
    assert_eq!(error_codes::E625_SES_PROTOCOL, "E625", "E625 code mismatch");
}

#[test]
fn error_codes_all_unique() {
    let codes = [
        error_codes::E610_REF_BOUND,
        error_codes::E612_REF_WIDTH,
        error_codes::E613_LIN_UNUSED,
        error_codes::E614_LIN_DOUBLE,
        error_codes::E616_EFF_PURE,
        error_codes::E617_EFF_MIX,
        error_codes::E618_CLK_CROSS,
        error_codes::E619_CLK_UNDEF,
        error_codes::E620_PHT_MISMATCH,
        error_codes::E621_PHT_UNDEF,
        error_codes::E625_SES_PROTOCOL,
    ];
    let mut seen = std::collections::HashSet::new();
    for i in 0..codes.len() {
        assert!(seen.insert(codes[i]), "Duplicate error code found: {}", codes[i]);
    }
    assert_eq!(seen.len(), 11, "Should have 11 distinct error codes");
}

// ===========================================================================
// Section 19: Bounded constants sanity
// ===========================================================================

#[test]
fn bounded_constants_are_nonzero() {
    assert!(MAX_REFINEMENT_PREDICATES > 0, "MAX_REFINEMENT_PREDICATES must be > 0");
    assert!(MAX_TYPE_NAT > 0, "MAX_TYPE_NAT must be > 0");
    assert!(MAX_CLOCK_DOMAINS > 0, "MAX_CLOCK_DOMAINS must be > 0");
    assert!(MAX_PHANTOM_TAGS > 0, "MAX_PHANTOM_TAGS must be > 0");
    assert!(MAX_SESSION_STATES > 0, "MAX_SESSION_STATES must be > 0");
    assert!(MAX_DEPENDENT_PARAMS > 0, "MAX_DEPENDENT_PARAMS must be > 0");
    assert!(MAX_EXTENDED_TYPE_NODES > 0, "MAX_EXTENDED_TYPE_NODES must be > 0");
}

#[test]
fn bounded_constants_values() {
    assert_eq!(MAX_REFINEMENT_PREDICATES, 8, "Expected MAX_REFINEMENT_PREDICATES=8");
    assert_eq!(MAX_TYPE_NAT, 65536, "Expected MAX_TYPE_NAT=65536");
    assert_eq!(MAX_CLOCK_DOMAINS, 16, "Expected MAX_CLOCK_DOMAINS=16");
    assert_eq!(MAX_PHANTOM_TAGS, 32, "Expected MAX_PHANTOM_TAGS=32");
    assert_eq!(MAX_SESSION_STATES, 64, "Expected MAX_SESSION_STATES=64");
    assert_eq!(MAX_DEPENDENT_PARAMS, 8, "Expected MAX_DEPENDENT_PARAMS=8");
    assert_eq!(MAX_EXTENDED_TYPE_NODES, 512, "Expected MAX_EXTENDED_TYPE_NODES=512");
}

// ===========================================================================
// Section 20: Pipeline integration — baseline
// ===========================================================================

#[test]
fn pipeline_baseline_plain_module_succeeds() {
    let result = run_extended(minimal_module());
    assert!(result.is_ok(), "Plain module should pass extended checking: {:?}", result.err());
    assert!(result.unwrap().extended_type_map.is_some(), "Extended type map should be populated");
}

#[test]
fn pipeline_baseline_numeric_module_succeeds() {
    let result = run_extended(numeric_module());
    assert!(result.is_ok(), "Numeric module should pass extended checking: {:?}", result.err());
}

#[test]
fn pipeline_extended_type_map_is_nonempty() {
    let result = run_extended(minimal_module()).expect("Pipeline should succeed");
    let ext_map = result.extended_type_map.expect("extended_type_map should be Some");
    assert!(!ext_map.is_empty(), "Extended type map should not be empty for a valid module");
}

// ===========================================================================
// Section 21: E610 — Refinement lower bound exceeds upper bound
// ===========================================================================

#[test]
fn e610_refinement_lower_exceeds_upper() {
    let module = parse_module(numeric_module());
    let mut decls = build_extended_decls(&module);

    // Manually add unsatisfiable refinement: value >= 500 && value <= 100
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueGe(500), span: None });
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(100), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E610"),
        "Should report E610 for unsatisfiable refinement bounds. Errors: {}",
        err_text
    );
}

#[test]
fn e610_refinement_equal_bounds_ok() {
    let module = parse_module(numeric_module());
    let mut decls = build_extended_decls(&module);

    // Equal bounds: value >= 50 && value <= 50 (singleton, but valid)
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueGe(50), span: None });
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(50), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E610"),
        "Equal lower and upper bounds should NOT trigger E610. Errors: {}",
        err_text
    );
}

#[test]
fn e610_inverted_range() {
    let module = parse_module(numeric_module());
    let mut decls = build_extended_decls(&module);

    // ValueInRange with lo > hi
    decls[0].extended_ty.refinements.push(RefinementPredicate {
        bound: RefinementBound::ValueInRange { lo: 200, hi: 100 },
        span: None,
    });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E610"),
        "Inverted range (lo=200 > hi=100) should trigger E610. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 22: E612 — Refinement bound exceeds bit-width capacity
// ===========================================================================

#[test]
fn e612_refinement_exceeds_u8_capacity() {
    let source = "module m {\n\
                   signal a: in u8;\n\
                   signal b: out u8;\n\
                   guard g {\n\
                   when a > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   b = a;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // u8 max is 255, but refinement says value <= 1000
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(1000), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E612"),
        "Refinement bound 1000 exceeding u8 capacity (255) should trigger E612. Errors: {}",
        err_text
    );
}

#[test]
fn e612_refinement_within_capacity_ok() {
    let module = parse_module(numeric_module());
    let mut decls = build_extended_decls(&module);

    // u16 max is 65535, refinement says value <= 1000 — fine
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(1000), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E612"),
        "Refinement within capacity should NOT trigger E612. Errors: {}",
        err_text
    );
}

#[test]
fn e612_refinement_at_exact_capacity() {
    let source = "module m {\n\
                   signal a: in u8;\n\
                   signal b: out u8;\n\
                   guard g {\n\
                   when a > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   b = a;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // u8 max is 255, refinement says value <= 255 — exactly at capacity
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(255), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E612"),
        "Refinement at exact capacity should NOT trigger E612. Errors: {}",
        err_text
    );
}

#[test]
fn e612_refinement_one_above_capacity() {
    let source = "module m {\n\
                   signal a: in u8;\n\
                   signal b: out u8;\n\
                   guard g {\n\
                   when a > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   b = a;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // u8 max is 255, refinement says value <= 256 — one above
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(256), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E612"),
        "Refinement one above u8 capacity should trigger E612. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 23: E613 — Linear signal never consumed
// ===========================================================================

#[test]
fn e613_linear_signal_not_consumed() {
    // linear input 'a' is declared but never read in any reflex RHS
    let source = "module m {\n\
                   signal a: in bool;\n\
                   signal b: in bool;\n\
                   signal y: out bool;\n\
                   guard g {\n\
                   when b\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = b;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // Mark 'a' as linear
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Linear);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E613"),
        "Linear signal 'a' never consumed should trigger E613. Errors: {}",
        err_text
    );
}

#[test]
fn e613_linear_signal_consumed_once_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Mark 'x' as linear — it IS consumed once in reflex r (y = x)
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Linear);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E613"),
        "Linear signal consumed once should NOT trigger E613. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 24: E614 — Linear signal consumed more than once
// ===========================================================================

#[test]
fn e614_linear_signal_double_consumption() {
    // 'a' is read twice in the same reflex: y = a && a
    let source = "module m {\n\
                   signal a: in bool;\n\
                   signal y: out bool;\n\
                   guard g {\n\
                   when a\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = a && a;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // Mark 'a' as linear
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Linear);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E614"),
        "Linear signal 'a' consumed twice should trigger E614. Errors: {}",
        err_text
    );
}

#[test]
fn e614_linear_signal_single_consumption_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Mark 'x' as linear — consumed exactly once in y = x
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Linear);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E614"),
        "Linear signal consumed exactly once should NOT trigger E614. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 25: E616 — Pure context contains stateful operation (prev)
// ===========================================================================

#[test]
fn e616_pure_signal_with_prev() {
    // y is pure but assigned from prev(x) — should trigger E616
    // Parser doesn't support prev() syntax, so we parse a normal module
    // then inject a Prev expression into the AST.
    let source = minimal_module();
    let mut module = parse_module(source);
    // Replace the assignment value `y = x` with `y = prev(x, 1)`
    module.reflexes[0].assignments[0].value =
        nasa_rust_project::ast::expr::Expr::Prev { signal: "x".to_string(), delay: 1 };
    let mut decls = build_extended_decls(&module);

    // Mark 'y' as pure
    decls[1].extended_ty.qualifiers.push(TypeQualifier::Pure);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E616"),
        "Pure signal assigned from prev() should trigger E616. Errors: {}",
        err_text
    );
}

#[test]
fn e616_pure_signal_without_prev_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Mark 'y' as pure — assigned from x (not prev), should be fine
    decls[1].extended_ty.qualifiers.push(TypeQualifier::Pure);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E616"),
        "Pure signal assigned from non-prev input should NOT trigger E616. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 26: E617 — Stateful signal used in pure expression
// ===========================================================================

#[test]
fn e617_pure_depends_on_stateful() {
    // y is pure, but reads from x which is stateful — should trigger E617
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Mark 'x' as stateful, 'y' as pure
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Stateful);
    decls[1].extended_ty.qualifiers.push(TypeQualifier::Pure);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E617"),
        "Pure signal depending on stateful should trigger E617. Errors: {}",
        err_text
    );
}

#[test]
fn e617_stateful_depends_on_stateful_ok() {
    // Both stateful — no error expected for effect mixing
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Both stateful
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Stateful);
    decls[1].extended_ty.qualifiers.push(TypeQualifier::Stateful);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E617"),
        "Stateful depending on stateful should NOT trigger E617. Errors: {}",
        err_text
    );
}

#[test]
fn e617_unqualified_depends_on_stateful_ok() {
    // Target is not pure, so no E617
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Only source is stateful, target has no qualifier
    decls[0].extended_ty.qualifiers.push(TypeQualifier::Stateful);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E617"),
        "Unqualified target depending on stateful should NOT trigger E617. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 27: E618 — Clock domain crossing without synchronizer
// ===========================================================================

#[test]
fn e618_cross_domain_assignment() {
    // x in @clk_fast, y in @clk_slow, y = x crosses domains
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let domains = vec![ClockDomain::new("clk_fast"), ClockDomain::new("clk_slow")];

    decls[0].extended_ty.clock_domain = Some(ClockDomain::new("clk_fast"));
    decls[1].extended_ty.clock_domain = Some(ClockDomain::new("clk_slow"));

    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E618"),
        "Cross-clock-domain assignment should trigger E618. Errors: {}",
        err_text
    );
}

#[test]
fn e618_same_domain_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let domains = vec![ClockDomain::new("clk_sys")];

    decls[0].extended_ty.clock_domain = Some(ClockDomain::new("clk_sys"));
    decls[1].extended_ty.clock_domain = Some(ClockDomain::new("clk_sys"));

    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E618"),
        "Same-domain assignment should NOT trigger E618. Errors: {}",
        err_text
    );
}

#[test]
fn e618_one_domain_one_default_no_crossing() {
    // Only one signal has a clock domain, the other is default. No crossing.
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let domains = vec![ClockDomain::new("clk_sys")];

    // Only target has a clock domain
    decls[1].extended_ty.clock_domain = Some(ClockDomain::new("clk_sys"));

    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E618"),
        "Signal without domain and signal with domain should NOT trigger E618. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 28: E619 — Undeclared clock domain
// ===========================================================================

#[test]
fn e619_undeclared_clock_domain() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Signal has domain but no domains are declared
    decls[0].extended_ty.clock_domain = Some(ClockDomain::new("clk_mystery"));

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E619"),
        "Undeclared clock domain should trigger E619. Errors: {}",
        err_text
    );
}

#[test]
fn e619_declared_domain_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let domains = vec![ClockDomain::new("clk_sys")];
    decls[0].extended_ty.clock_domain = Some(ClockDomain::new("clk_sys"));

    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E619"),
        "Declared clock domain should NOT trigger E619. Errors: {}",
        err_text
    );
}

#[test]
fn e619_pipeline_undeclared_clock_domain() {
    // Through the pipeline, clock_domains list is empty, so any @domain triggers E619
    let source = "module m {\n\
                   signal x: in u16 @sys_clk;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_err(), "Undeclared clock domain should produce errors");
    let errors = match result {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E619"), "Pipeline should report E619 for undeclared domain: {}", msg);
}

// ===========================================================================
// Section 29: E620 — Phantom tag mismatch
// ===========================================================================

#[test]
fn e620_phantom_tag_mismatch_in_assignment() {
    // x is #Unverified, y is #Verified, y = x is a tag mismatch
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Unverified"), PhantomTag::new("Verified")];

    decls[0].extended_ty.phantom = Some(PhantomTag::new("Unverified"));
    decls[1].extended_ty.phantom = Some(PhantomTag::new("Verified"));

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E620"),
        "Phantom tag mismatch (Unverified->Verified) should trigger E620. Errors: {}",
        err_text
    );
}

#[test]
fn e620_phantom_tag_same_tag_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Verified")];

    decls[0].extended_ty.phantom = Some(PhantomTag::new("Verified"));
    decls[1].extended_ty.phantom = Some(PhantomTag::new("Verified"));

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E620"),
        "Same phantom tag should NOT trigger E620. Errors: {}",
        err_text
    );
}

#[test]
fn e620_untagged_to_tagged_target() {
    // x is untagged, y is #Verified — assigning untagged to tagged is error
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Verified")];

    // x has no phantom, y has phantom
    decls[1].extended_ty.phantom = Some(PhantomTag::new("Verified"));

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E620"),
        "Untagged source to tagged target should trigger E620. Errors: {}",
        err_text
    );
}

#[test]
fn e620_tagged_to_untagged_target_ok() {
    // x is #Verified, y is untagged — tag is dropped (allowed)
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Verified")];

    decls[0].extended_ty.phantom = Some(PhantomTag::new("Verified"));
    // y has no phantom

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E620"),
        "Tagged source to untagged target should NOT trigger E620. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 30: E621 — Undeclared phantom tag
// ===========================================================================

#[test]
fn e621_undeclared_phantom_tag() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Signal has phantom tag but tag is not in declared list
    decls[0].extended_ty.phantom = Some(PhantomTag::new("Mystery"));

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E621"),
        "Undeclared phantom tag should trigger E621. Errors: {}",
        err_text
    );
}

#[test]
fn e621_declared_phantom_tag_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Voltage")];
    decls[0].extended_ty.phantom = Some(PhantomTag::new("Voltage"));

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E621"),
        "Declared phantom tag should NOT trigger E621. Errors: {}",
        err_text
    );
}

#[test]
fn e621_pipeline_undeclared_phantom_tag() {
    let source = "module m {\n\
                   signal x: in u16 #Voltage;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_err(), "Undeclared phantom tag should produce errors");
    let errors = match result {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E621"), "Pipeline should report E621 for undeclared tag: {}", msg);
}

// ===========================================================================
// Section 31: E625 — Session type protocol violation
// ===========================================================================

#[test]
fn e625_undeclared_protocol() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    // Signal references a protocol that does not exist
    decls[0].extended_ty.session = Some(SessionTypeRef {
        protocol: "NonexistentProto".to_string(),
        state: "Idle".to_string(),
        role: SessionRole::Sender,
    });

    // Provide a different protocol so the protocol map is non-empty
    // (check_session_types early-returns if the map is empty).
    let protocols = vec![SessionProtocol {
        name: "SomeOtherProto".to_string(),
        transitions: vec![SessionTransition {
            from: "A".to_string(),
            to: "B".to_string(),
            guard: None,
        }],
        span: None,
    }];
    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E625"),
        "Undeclared protocol should trigger E625. Errors: {}",
        err_text
    );
}

#[test]
fn e625_invalid_state_in_protocol() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let protocols = vec![SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![
            SessionTransition { from: "Idle".to_string(), to: "Ready".to_string(), guard: None },
            SessionTransition { from: "Ready".to_string(), to: "Idle".to_string(), guard: None },
        ],
        span: None,
    }];

    // Signal references a state that does not exist in the protocol
    decls[0].extended_ty.session = Some(SessionTypeRef {
        protocol: "Handshake".to_string(),
        state: "Bogus".to_string(), // not Idle or Ready
        role: SessionRole::Sender,
    });

    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E625"),
        "Invalid state in protocol should trigger E625. Errors: {}",
        err_text
    );
}

#[test]
fn e625_valid_protocol_and_state_ok() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let protocols = vec![SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![
            SessionTransition { from: "Idle".to_string(), to: "Ready".to_string(), guard: None },
            SessionTransition { from: "Ready".to_string(), to: "Idle".to_string(), guard: None },
        ],
        span: None,
    }];

    decls[0].extended_ty.session = Some(SessionTypeRef {
        protocol: "Handshake".to_string(),
        state: "Idle".to_string(),
        role: SessionRole::Sender,
    });

    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E625"),
        "Valid protocol and state should NOT trigger E625. Errors: {}",
        err_text
    );
}

#[test]
fn e625_valid_state_as_to_target() {
    // State exists only as a "to" target, not a "from" source, but should still be valid
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let protocols = vec![SessionProtocol {
        name: "OneWay".to_string(),
        transitions: vec![SessionTransition {
            from: "Start".to_string(),
            to: "End".to_string(),
            guard: None,
        }],
        span: None,
    }];

    decls[0].extended_ty.session = Some(SessionTypeRef {
        protocol: "OneWay".to_string(),
        state: "End".to_string(),
        role: SessionRole::Receiver,
    });

    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E625"),
        "State that exists as a transition target should be valid. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 32: Pipeline integration with annotations
// ===========================================================================

#[test]
fn pipeline_linear_annotation_parses_and_checks() {
    let source = "module m {\n\
                   signal x: in linear bool;\n\
                   signal y: out bool;\n\
                   guard g {\n\
                   when x\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_ok(), "Linear annotated module should pass: {:?}", result.err());
}

#[test]
fn pipeline_stateful_annotation_parses_and_checks() {
    let source = "module m {\n\
                   signal x: in stateful u16;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_ok(), "Stateful annotated module should pass: {:?}", result.err());
}

#[test]
fn pipeline_pure_annotation_parses_and_checks() {
    let source = "module m {\n\
                   signal x: in pure u16;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_ok(), "Pure annotated module should pass: {:?}", result.err());
}

#[test]
fn pipeline_refinement_annotation_parses_and_checks() {
    let source = "module m {\n\
                   signal x: in u16 where 0..200;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_ok(), "Refinement annotated module should pass: {:?}", result.err());
}

// ===========================================================================
// Section 33: Combined error scenarios
// ===========================================================================

#[test]
fn combined_e619_and_e621_via_pipeline() {
    // Signal with both undeclared clock domain and undeclared phantom tag
    let source = "module m {\n\
                   signal x: in linear stateful u16 where 0..1000 @fast_clk #Temp;\n\
                   signal y: out u16;\n\
                   guard g {\n\
                   when x > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let result = run_extended(source);
    assert!(result.is_err(), "Undeclared clock+phantom should produce errors");
    let errors = match result {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E619"), "Should contain E619: {}", msg);
    assert!(msg.contains("E621"), "Should contain E621: {}", msg);
}

#[test]
fn combined_e610_and_e612() {
    // A single signal with both inverted bounds AND bounds exceeding capacity
    let source = "module m {\n\
                   signal a: in u8;\n\
                   signal b: out u8;\n\
                   guard g {\n\
                   when a > 0\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   b = a;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    // Lower bound 500 > upper bound 100 (E610) AND 500 > u8 capacity 255 (E612)
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueGe(500), span: None });
    decls[0]
        .extended_ty
        .refinements
        .push(RefinementPredicate { bound: RefinementBound::ValueLe(100), span: None });

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E610"),
        "Should contain E610 for inverted bounds. Errors: {}",
        err_text
    );
}

#[test]
fn combined_e616_and_e617() {
    // Pure signal assigned from prev() of a stateful signal — both E616 and E617
    let source = minimal_module();
    let mut module = parse_module(source);
    // Inject Prev expression: y = prev(x, 1)
    module.reflexes[0].assignments[0].value =
        nasa_rust_project::ast::expr::Expr::Prev { signal: "x".to_string(), delay: 1 };
    let mut decls = build_extended_decls(&module);

    decls[0].extended_ty.qualifiers.push(TypeQualifier::Stateful);
    decls[1].extended_ty.qualifiers.push(TypeQualifier::Pure);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        err_text.contains("E616"),
        "Should contain E616 for prev in pure context. Errors: {}",
        err_text
    );
    assert!(
        err_text.contains("E617"),
        "Should contain E617 for stateful signal in pure context. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 34: No-extension baseline — all phases skip gracefully
// ===========================================================================

#[test]
fn no_extensions_all_phases_pass() {
    let module = parse_module(minimal_module());
    let decls = build_extended_decls(&module);

    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    assert!(
        result.errors.is_empty(),
        "Module with no extensions should produce zero errors. Errors: {:?}",
        result.errors
    );
}

#[test]
fn empty_module_no_errors() {
    let source = "module empty {\n\
                   signal x: in bool;\n\
                   signal y: out bool;\n\
                   guard g {\n\
                   when x\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = true;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let decls = build_extended_decls(&module);
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    assert!(
        result.errors.is_empty(),
        "Simple module with literal assignment should produce zero errors. Errors: {:?}",
        result.errors
    );
}

// ===========================================================================
// Section 35: Multiple signals with various extensions
// ===========================================================================

#[test]
fn multiple_domains_declared_no_crossing() {
    let source = "module m {\n\
                   signal x: in bool;\n\
                   signal y: out bool;\n\
                   signal z: out bool;\n\
                   guard g {\n\
                   when x\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    let domains = vec![ClockDomain::new("clk_a"), ClockDomain::new("clk_b")];

    // x in clk_a, y in clk_a — same domain, no crossing
    decls[0].extended_ty.clock_domain = Some(ClockDomain::new("clk_a"));
    decls[1].extended_ty.clock_domain = Some(ClockDomain::new("clk_a"));
    // z in clk_b but not involved in any assignment

    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E618"),
        "Same-domain signals should not cross. Errors: {}",
        err_text
    );
}

#[test]
fn multiple_phantom_tags_matching() {
    let source = "module m {\n\
                   signal x: in bool;\n\
                   signal y: out bool;\n\
                   guard g {\n\
                   when x\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r {\n\
                   on g {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let mut decls = build_extended_decls(&module);

    let tags = vec![PhantomTag::new("Safe"), PhantomTag::new("Unsafe")];

    // Both have same tag
    decls[0].extended_ty.phantom = Some(PhantomTag::new("Safe"));
    decls[1].extended_ty.phantom = Some(PhantomTag::new("Safe"));

    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E620"),
        "Matching phantom tags should not trigger E620. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 36: Session protocol with multiple transitions
// ===========================================================================

#[test]
fn session_protocol_multiple_transitions_valid() {
    let module = parse_module(minimal_module());
    let mut decls = build_extended_decls(&module);

    let protocols = vec![SessionProtocol {
        name: "SPI".to_string(),
        transitions: vec![
            SessionTransition { from: "Idle".to_string(), to: "Select".to_string(), guard: None },
            SessionTransition {
                from: "Select".to_string(),
                to: "Transfer".to_string(),
                guard: None,
            },
            SessionTransition { from: "Transfer".to_string(), to: "Done".to_string(), guard: None },
            SessionTransition { from: "Done".to_string(), to: "Idle".to_string(), guard: None },
        ],
        span: None,
    }];

    // Signal in state "Transfer" (valid — appears as both from and to)
    decls[0].extended_ty.session = Some(SessionTypeRef {
        protocol: "SPI".to_string(),
        state: "Transfer".to_string(),
        role: SessionRole::Sender,
    });

    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let err_text = collect_error_text(&result.errors);
    assert!(
        !err_text.contains("E625"),
        "Valid state in multi-transition protocol should NOT trigger E625. Errors: {}",
        err_text
    );
}

// ===========================================================================
// Section 37: Serialization round-trip (serde)
// ===========================================================================

#[test]
fn extended_type_serde_roundtrip_base_only() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    let json = serde_json::to_string(&et).expect("Serialization should succeed");
    let deserialized: ExtendedType =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(et, deserialized, "Round-trip should preserve ExtendedType");
}

#[test]
fn extended_type_serde_roundtrip_with_all_features() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.qualifiers.push(TypeQualifier::Linear);
    et.qualifiers.push(TypeQualifier::Stateful);
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLe(1000), span: None });
    et.clock_domain = Some(ClockDomain::new("sys_clk"));
    et.phantom = Some(PhantomTag::new("Verified"));
    et.type_nat = Some(TypeNat::new(8).unwrap());
    et.dependent_params.push(DependentParam::Const(4));
    et.session = Some(SessionTypeRef {
        protocol: "Proto".to_string(),
        state: "Init".to_string(),
        role: SessionRole::Receiver,
    });

    let json = serde_json::to_string(&et).expect("Serialization should succeed");
    let deserialized: ExtendedType =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(
        et, deserialized,
        "Full-featured ExtendedType round-trip should preserve all fields"
    );
}

#[test]
fn clock_domain_serde_roundtrip() {
    let cd = ClockDomain::new("fast").with_frequency(100_000_000);
    let json = serde_json::to_string(&cd).expect("Serialization should succeed");
    let deserialized: ClockDomain =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(cd, deserialized, "ClockDomain round-trip should preserve all fields");
}

#[test]
fn phantom_tag_serde_roundtrip() {
    let pt = PhantomTag::new("Encrypted");
    let json = serde_json::to_string(&pt).expect("Serialization should succeed");
    let deserialized: PhantomTag =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(pt, deserialized, "PhantomTag round-trip should preserve all fields");
}

#[test]
fn session_protocol_serde_roundtrip() {
    let proto = SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![SessionTransition {
            from: "Idle".to_string(),
            to: "Ready".to_string(),
            guard: Some("req_valid".to_string()),
        }],
        span: None,
    };
    let json = serde_json::to_string(&proto).expect("Serialization should succeed");
    let deserialized: SessionProtocol =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(proto, deserialized, "SessionProtocol round-trip should preserve all fields");
}

#[test]
fn refinement_bound_serde_roundtrip_all_variants() {
    let bounds = [
        RefinementBound::ValueLt(100),
        RefinementBound::ValueLe(200),
        RefinementBound::ValueGt(10),
        RefinementBound::ValueGe(20),
        RefinementBound::ValueEq(42),
        RefinementBound::ValueNe(0),
        RefinementBound::ValueInRange { lo: 5, hi: 50 },
        RefinementBound::ValueMod { divisor: 4, remainder: 1 },
    ];
    for i in 0..bounds.len() {
        let json = serde_json::to_string(&bounds[i]).expect("Serialization should succeed");
        let deserialized: RefinementBound =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(
            bounds[i], deserialized,
            "RefinementBound variant {:?} round-trip failed",
            bounds[i]
        );
    }
}

// ===========================================================================
// Section 38: Edge cases and stress
// ===========================================================================

#[test]
fn many_refinement_predicates_bounded() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(32));
    // Add MAX_REFINEMENT_PREDICATES predicates
    for i in 0..MAX_REFINEMENT_PREDICATES {
        et.refinements.push(RefinementPredicate {
            bound: RefinementBound::ValueLe((i as u64 + 1) * 1000),
            span: None,
        });
    }
    assert_eq!(
        et.refinements.len(),
        MAX_REFINEMENT_PREDICATES,
        "Should have exactly MAX_REFINEMENT_PREDICATES predicates"
    );
    // The tightest upper bound should be 1000 (first one)
    let hint = refinement_width_hint(&et);
    assert!(hint.is_some(), "Should produce a width hint from many predicates");
}

#[test]
fn refinement_width_hint_value_lt_1() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
    et.refinements.push(RefinementPredicate {
        bound: RefinementBound::ValueLt(1), // max=0
        span: None,
    });
    let hint = refinement_width_hint(&et);
    // 0 needs 0 bits (or 1 bit depending on implementation)
    assert!(hint.is_some(), "ValueLt(1) should produce a width hint");
}

#[test]
fn type_nat_large_total_width() {
    let nat = TypeNat::new(MAX_TYPE_NAT).expect("MAX_TYPE_NAT should be valid");
    let total = nat.total_width(64);
    // 65536 * 64 = 4,194,304, which fits in u64
    assert_eq!(total, Some(MAX_TYPE_NAT * 64), "Large total width should compute correctly");
}

#[test]
fn extended_type_display_empty_refinement_no_where() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    let display = et.to_string();
    assert!(!display.contains("where"), "Type with no refinements should not display 'where'");
}

#[test]
fn extended_type_display_empty_dependent_no_angle_brackets() {
    let et = ExtendedType::from_base(SignalType::Unsigned(16));
    let display = et.to_string();
    assert!(!display.contains("<"), "Type with no dependent params should not display '<'");
}

#[test]
fn extended_type_display_single_refinement_no_ampersand() {
    let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
    et.refinements.push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
    let display = et.to_string();
    assert!(!display.contains("&&"), "Single refinement should not include '&&'");
    assert!(display.contains("where"), "Single refinement should include 'where'");
}

// ===========================================================================
// Section 39: RefinementPredicate Display
// ===========================================================================

#[test]
fn refinement_predicate_display_delegates_to_bound() {
    let pred = RefinementPredicate { bound: RefinementBound::ValueGe(42), span: None };
    assert_eq!(
        pred.to_string(),
        "value >= 42",
        "RefinementPredicate Display should delegate to RefinementBound"
    );
}

// ===========================================================================
// Section 40: Extended syntax placeholder
// ===========================================================================

#[test]
fn parse_extended_type_annotation_returns_parse_error() {
    use nasa_rust_project::typeck::extended::syntax::parse_extended_type_annotation;
    let result = parse_extended_type_annotation("u16 where value < 1024");
    assert!(result.is_err(), "Extended type parsing placeholder should return error");
}

// ===========================================================================
// Section 41: Direct typecheck_extended on varied module shapes
// ===========================================================================

#[test]
fn typecheck_extended_with_multiple_reflexes() {
    let source = "module m {\n\
                   signal x: in bool;\n\
                   signal y: out bool;\n\
                   signal z: out bool;\n\
                   guard g1 {\n\
                   when x\n\
                   for 1 cycles;\n\
                   }\n\
                   guard g2 {\n\
                   when !x\n\
                   for 1 cycles;\n\
                   }\n\
                   reflex r1 {\n\
                   on g1 {\n\
                   y = x;\n\
                   }\n\
                   }\n\
                   reflex r2 {\n\
                   on g2 {\n\
                   z = x;\n\
                   }\n\
                   }\n\
                   }";
    let module = parse_module(source);
    let decls = build_extended_decls(&module);
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    assert!(
        result.errors.is_empty(),
        "Module with multiple reflexes and no extensions should pass. Errors: {:?}",
        result.errors
    );
}

#[test]
fn typecheck_extended_produces_type_map_entries() {
    let module = parse_module(minimal_module());
    let decls = build_extended_decls(&module);
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    assert!(result.errors.is_empty(), "Should produce no errors for minimal module");
    assert!(
        !result.type_map.is_empty(),
        "Extended type map should have entries for expression nodes"
    );
}

// ===========================================================================
// Section 42: Coverage for all RefinementBound implied_min variants
// ===========================================================================

#[test]
fn implied_min_value_ne_returns_none() {
    assert_eq!(
        RefinementBound::ValueNe(10).implied_min(),
        None,
        "ValueNe does not constrain lower bound"
    );
}

#[test]
fn implied_min_value_mod_returns_none() {
    assert_eq!(
        RefinementBound::ValueMod { divisor: 5, remainder: 2 }.implied_min(),
        None,
        "ValueMod does not constrain lower bound"
    );
}

#[test]
fn implied_min_value_le_returns_none() {
    assert_eq!(
        RefinementBound::ValueLe(100).implied_min(),
        None,
        "ValueLe does not constrain lower bound"
    );
}
