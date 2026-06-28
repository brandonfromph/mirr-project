#![forbid(unsafe_code)]

use mirrc::ast::types::{
    packet_type, surface_type, EffectQualifier, Linearity, SignalType,
};

#[test]
fn packet_type_is_struct() {
    let pt = packet_type();
    assert!(matches!(pt, SignalType::Struct { .. }));
}

#[test]
fn surface_type_is_struct() {
    let st = surface_type();
    assert!(matches!(st, SignalType::Struct { .. }));
}

#[test]
fn format_fixed_point() {
    let fp = SignalType::FixedPoint { total_bits: 16, frac_bits: 8 };
    assert_eq!(fp.to_string(), "fixed<16,8>");
}

#[test]
fn format_fifo() {
    let fifo = SignalType::Fifo {
        element: Box::new(SignalType::Unsigned(8)),
        depth: 16,
    };
    assert_eq!(fifo.to_string(), "fifo<u8, 16>");
}

#[test]
fn width_and_signed_for_signed() {
    let s = SignalType::Signed(32);
    let (w, sign) = s.width_and_signed();
    assert_eq!(w, 32);
    assert_eq!(sign, true);
}

#[test]
fn linearity_is_unrestricted() {
    let lin_un = Linearity::Unrestricted;
    assert!(lin_un.is_unrestricted());
    
    let lin_lin = Linearity::Linear;
    assert!(!lin_lin.is_unrestricted());
}

#[test]
fn effect_qualifier_is_unspecified() {
    let eff_un = EffectQualifier::Unspecified;
    assert!(eff_un.is_unspecified());
    
    let eff_state = EffectQualifier::Stateful;
    assert!(!eff_state.is_unspecified());
}
