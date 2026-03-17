//! MEGA-1 Extended Type System for MIRR.
//!
//! Extends the core `SignalType` with refinement types, linear types, effect
//! types, clock domain qualifiers, phantom provenance tags, type-level
//! naturals, dependent types, and session types.
//!
//! All features are **opt-in**: existing MIRR programs that use only
//! `SignalType::{Bool, Unsigned, Signed}` continue to work unchanged.
//! The extended system wraps `SignalType` in an `ExtendedType` struct that
//! carries optional metadata for each new type feature.
//!
//! Design constraints (NASA P10 / MIRR policy):
//! - `#![forbid(unsafe_code)]`
//! - No recursion — all traversals use explicit bounded loops
//! - All loops bounded by `MAX_*` constants
//! - All new types derive `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`
//! - Backward-compatible: new optional fields use `#[serde(default)]`
//!
//! Error codes: E610–E625 (see error catalogue below).
//!
//! ## Error Code Allocation (MEGA-1)
//!
//! | Code | Rule | Description |
//! |------|------|-------------|
//! | E610 | REF-BOUND | Refinement lower bound exceeds upper bound |
//! | E612 | REF-WIDTH | Refinement bound exceeds declared bit-width capacity |
//! | E613 | LIN-UNUSED | Linear signal declared but never consumed |
//! | E614 | LIN-DOUBLE | Linear signal consumed more than once in a clock cycle |
//! | E616 | EFF-PURE | Pure (combinational) context contains stateful operation |
//! | E617 | EFF-MIX | Effect mismatch: stateful signal used in pure expression |
//! | E618 | CLK-CROSS | Clock domain crossing without synchronizer |
//! | E619 | CLK-UNDEF | Reference to undeclared clock domain |
//! | E620 | PHT-MISMATCH | Phantom tag mismatch in assignment or comparison |
//! | E621 | PHT-UNDEF | Reference to undeclared phantom tag |
//! | E625 | SES-PROTOCOL | Session type protocol violation (unexpected message) |

#![forbid(unsafe_code)]

mod checks;
mod domain_checks;
mod emit;
mod qualifiers;
mod types;

// Re-export everything so external code sees no change.
pub use checks::*;
pub use emit::*;
pub use qualifiers::*;
pub use types::*;

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::SignalType;

    #[test]
    fn extended_type_from_base_is_base_only() {
        let et = ExtendedType::from_base(SignalType::Unsigned(16));
        assert!(et.is_base_only());
        assert!(!et.is_linear());
        assert!(!et.is_pure());
        assert!(!et.is_stateful());
        assert_eq!(et.clock_domain_name(), None);
    }

    #[test]
    fn extended_type_with_qualifiers() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
        et.qualifiers.push(TypeQualifier::Linear);
        et.qualifiers.push(TypeQualifier::Pure);
        assert!(!et.is_base_only());
        assert!(et.is_linear());
        assert!(et.is_pure());
    }

    #[test]
    fn refinement_bound_satisfied() {
        assert!(RefinementBound::ValueLt(1024).satisfied_by(1023));
        assert!(!RefinementBound::ValueLt(1024).satisfied_by(1024));
        assert!(RefinementBound::ValueGe(10).satisfied_by(10));
        assert!(!RefinementBound::ValueGe(10).satisfied_by(9));
        assert!(RefinementBound::ValueInRange { lo: 5, hi: 15 }.satisfied_by(10));
        assert!(!RefinementBound::ValueInRange { lo: 5, hi: 15 }.satisfied_by(16));
        assert!(RefinementBound::ValueMod { divisor: 4, remainder: 0 }.satisfied_by(8));
        assert!(!RefinementBound::ValueMod { divisor: 4, remainder: 0 }.satisfied_by(7));
    }

    #[test]
    fn refinement_bound_implied_max() {
        assert_eq!(RefinementBound::ValueLt(1024).implied_max(), Some(1023));
        assert_eq!(RefinementBound::ValueLe(1024).implied_max(), Some(1024));
        assert_eq!(RefinementBound::ValueGt(5).implied_max(), None);
        assert_eq!(RefinementBound::ValueInRange { lo: 10, hi: 200 }.implied_max(), Some(200));
    }

    #[test]
    fn refinement_width_hint_from_upper_bound() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
        let hint = refinement_width_hint(&et);
        // 1023 needs 10 bits
        assert_eq!(hint, Some(crate::width::types::Width(10)));
    }

    #[test]
    fn effective_width_with_refinement() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(256), span: None });
        // 255 needs 8 bits, which is less than declared 16
        assert_eq!(effective_width(&et), crate::width::types::Width(8));
    }

    #[test]
    fn effective_width_no_refinement() {
        let et = ExtendedType::from_base(SignalType::Unsigned(16));
        assert_eq!(effective_width(&et), crate::width::types::Width(16));
    }

    #[test]
    fn base_max_value_unsigned() {
        let et = ExtendedType::from_base(SignalType::Unsigned(8));
        assert_eq!(et.base_max_value(), Some(255));
    }

    #[test]
    fn base_max_value_bool() {
        let et = ExtendedType::from_base(SignalType::Bool);
        assert_eq!(et.base_max_value(), Some(1));
    }

    #[test]
    fn clock_domain_display() {
        let cd = ClockDomain::new("clk_fast").with_frequency(100_000_000);
        assert_eq!(cd.to_string(), "@clk_fast(100000000Hz)");
    }

    #[test]
    fn phantom_tag_display() {
        let pt = PhantomTag::new("Verified");
        assert_eq!(pt.to_string(), "#Verified");
    }

    #[test]
    fn type_nat_bounds() {
        assert!(TypeNat::new(65536).is_some());
        assert!(TypeNat::new(65537).is_none());
    }

    #[test]
    fn type_nat_total_width() {
        let nat = TypeNat::new(4).unwrap();
        assert_eq!(nat.total_width(8), Some(32)); // 4 * 8 = 32
    }

    #[test]
    fn extended_type_display() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(16));
        et.qualifiers.push(TypeQualifier::Linear);
        et.clock_domain = Some(ClockDomain::new("clk_fast"));
        et.phantom = Some(PhantomTag::new("Verified"));
        et.refinements
            .push(RefinementPredicate { bound: RefinementBound::ValueLt(1024), span: None });
        let display = et.to_string();
        assert!(display.contains("u16"));
        assert!(display.contains("linear"));
        assert!(display.contains("@clk_fast"));
        assert!(display.contains("#Verified"));
        assert!(display.contains("value < 1024"));
    }

    #[test]
    fn extended_signal_decl_from_legacy() {
        let legacy = crate::ast::program::SignalDecl {
            name: "sensor".to_string(),
            kind: crate::ast::types::SignalKind::Input,
            ty: crate::ast::types::ExtendedType::from_core(SignalType::Unsigned(16)),
            origin: None,
            span: None,
        };
        let ext = ExtendedSignalDecl::from_legacy(&legacy);
        assert_eq!(ext.ty, SignalType::Unsigned(16));
        assert!(ext.extended_ty.is_base_only());
    }

    #[test]
    fn hardware_mapping_synthesis_impact() {
        let base = ExtendedType::from_base(SignalType::Unsigned(8));
        assert!(!hardware_mapping::has_synthesis_impact(&base));

        let mut with_clock = ExtendedType::from_base(SignalType::Unsigned(8));
        with_clock.clock_domain = Some(ClockDomain::new("clk_fast"));
        assert!(hardware_mapping::has_synthesis_impact(&with_clock));

        let mut with_pure = ExtendedType::from_base(SignalType::Unsigned(8));
        with_pure.qualifiers.push(TypeQualifier::Pure);
        assert!(hardware_mapping::has_synthesis_impact(&with_pure));
    }

    #[test]
    fn firrtl_type_with_array() {
        let mut et = ExtendedType::from_base(SignalType::Unsigned(8));
        et.type_nat = Some(TypeNat::new(4).unwrap());
        assert_eq!(hardware_mapping::extended_firrtl_type(&et), "UInt<8>[4]");
    }

    #[test]
    fn firrtl_type_scalar() {
        let et = ExtendedType::from_base(SignalType::Signed(16));
        assert_eq!(hardware_mapping::extended_firrtl_type(&et), "SInt<16>");
    }

    #[test]
    fn session_role_display() {
        assert_eq!(SessionRole::Sender.to_string(), "sender");
        assert_eq!(SessionRole::Receiver.to_string(), "receiver");
    }

    #[test]
    fn dependent_param_display() {
        assert_eq!(DependentParam::Const(4).to_string(), "4");
        assert_eq!(DependentParam::Type(SignalType::Unsigned(8)).to_string(), "u8");
        assert_eq!(DependentParam::Phantom(PhantomTag::new("V")).to_string(), "#V");
    }

    #[test]
    fn refinement_bound_display() {
        assert_eq!(RefinementBound::ValueLt(1024).to_string(), "value < 1024");
        assert_eq!(
            RefinementBound::ValueInRange { lo: 10, hi: 200 }.to_string(),
            "value in 10..=200"
        );
        assert_eq!(
            RefinementBound::ValueMod { divisor: 4, remainder: 0 }.to_string(),
            "value % 4 == 0"
        );
    }

    #[test]
    fn error_codes_are_distinct() {
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
        for code in &codes {
            assert!(seen.insert(*code), "Duplicate error code: {}", code);
        }
        assert_eq!(codes.len(), 11);
    }
}
