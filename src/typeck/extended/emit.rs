//! Width inference integration, syntax definitions, and hardware mapping.
//!
//! Part of the MEGA-1 Extended Type System.

#![forbid(unsafe_code)]

use super::types::*;
use crate::ast::types::SignalType;

// ===========================================================================
// F) Refinement → FIRWINE width inference API
// ===========================================================================

/// Extract width hints from refinement bounds for the FIRWINE width inference pass.
///
/// Given an `ExtendedType`, computes the tightest upper-bound value implied
/// by all its refinement predicates, then converts that to a minimum bit-width.
///
/// Returns `None` if no refinement bounds constrain the upper range (the
/// existing width inference logic applies).
///
/// # Integration Point
///
/// This function is called by `width::constraint::generate_constraints` when
/// building the constraint set for a signal node. If a refinement-derived
/// width hint exists, it is used as an upper bound on the signal's width
/// variable, potentially allowing narrower hardware than the declared type.
///
/// Example:
/// ```text
/// signal x: out u16 where value < 1024;
/// ```
/// Declared width = 16 bits. Refinement says max value is 1023.
/// `min_bits_for(1023) = 10`. FIRWINE can infer `x` needs only 10 bits
/// (though the wire is still 16 bits for interface compatibility — the
/// optimization is that downstream logic can assume the top 6 bits are 0).
pub fn refinement_width_hint(extended_ty: &ExtendedType) -> Option<crate::width::types::Width> {
    if extended_ty.refinements.is_empty() {
        return None;
    }

    // Find the tightest upper bound across all predicates
    let mut tightest_max: Option<u64> = None;

    let mut pred_idx = 0usize;
    while pred_idx < extended_ty.refinements.len() && pred_idx < MAX_REFINEMENT_PREDICATES {
        let pred = &extended_ty.refinements[pred_idx];
        pred_idx += 1;

        if let Some(implied_max) = pred.bound.implied_max() {
            tightest_max =
                Some(tightest_max.map_or(implied_max, |current| current.min(implied_max)));
        }
    }

    tightest_max.map(crate::width::types::Width::min_bits_for)
}

/// Compute the refined width for a signal, taking the minimum of the
/// declared bit-width and the refinement-derived hint.
///
/// This is the primary API for downstream passes to query "what is the
/// effective width of this signal, considering refinements?"
///
/// Returns the declared width if no refinement narrows it.
pub fn effective_width(extended_ty: &ExtendedType) -> crate::width::types::Width {
    let declared = match extended_ty.base {
        SignalType::Bool => crate::width::types::Width(1),
        SignalType::Unsigned(w) | SignalType::Signed(w) => crate::width::types::Width(w),
    };

    match refinement_width_hint(extended_ty) {
        Some(hint) if hint.0 < declared.0 => hint,
        _ => declared,
    }
}

// ===========================================================================
// H) Parser syntax: token sequences for each type feature
// ===========================================================================

/// Describes the concrete syntax extensions for MEGA-1 type features.
///
/// This module does not contain runnable parser code (that lives in
/// `parser::module_parser`), but documents the token sequences that the
/// parser will recognize for each feature.
///
/// ## Refinement Types
///
/// ```text
/// signal x: out u16 where value < 1024;
/// signal y: out u8  where value >= 10 && value <= 200;
/// ```
///
/// Token sequence: `Ident("where") Ident("value") (Lt|Le|Gt|Ge|EqEq|BangEq) Integer`
/// Multiple predicates joined by `AmpAmp`.
///
/// ## Linear Types
///
/// ```text
/// signal x: out linear u16;
/// ```
///
/// Token sequence: `Ident("linear")` before the type name.
///
/// ## Effect Types
///
/// ```text
/// signal x: out pure u16;
/// signal y: internal stateful u32;
/// ```
///
/// Token sequence: `Ident("pure")` or `Ident("stateful")` before the type name.
///
/// ## Clock Domain Qualifiers
///
/// ```text
/// signal x: in u16 @clk_fast;
/// ```
///
/// Token sequence: `At Ident("clk_fast")` after the type.
/// Requires new `At` token (`@`) in the lexer.
///
/// ## Phantom Types
///
/// ```text
/// signal x: in u16 #Verified;
/// ```
///
/// Token sequence: `Hash Ident("Verified")` after the type.
/// Requires new `Hash` token (`#`) in the lexer.
///
/// ## Type-Level Naturals (Array Dimensions)
///
/// ```text
/// signal x: in u8[4];
/// ```
///
/// Token sequence: type `LBracket Integer RBracket` after the base type.
/// Requires new `LBracket`/`RBracket` tokens.
///
/// ## Dependent Types
///
/// ```text
/// signal x: in Vector<u8, 4>;
/// ```
///
/// Token sequence: `Ident Lt` type/integer params separated by `Comma` then `Gt`.
/// Reuses existing `Lt`/`Gt` tokens in a type-position context.
///
/// ## Session Types
///
/// ```text
/// signal x: out bool session Handshake::Idle;
/// ```
///
/// Token sequence: `Ident("session") Ident ColonColon Ident` after the type.
/// Requires new `ColonColon` token (`::`).
pub mod syntax {
    /// New tokens required by MEGA-1 type syntax.
    ///
    /// These extend the existing `Token` enum in `lexer/tokenizer.rs`.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ExtendedToken {
        /// `@` — clock domain prefix.
        At,
        /// `#` — phantom tag prefix.
        Hash,
        /// `[` — array dimension open.
        LBracket,
        /// `]` — array dimension close.
        RBracket,
        /// `::` — scope resolution (for session types).
        ColonColon,
        /// `,` — parameter separator (for dependent types).
        Comma,
        /// `where` keyword (for refinements).
        Where,
        /// `linear` keyword.
        Linear,
        /// `pure` keyword.
        KwPure,
        /// `stateful` keyword.
        KwStateful,
        /// `session` keyword.
        Session,
        /// `protocol` keyword (for protocol definitions).
        Protocol,
        /// `state` keyword (within protocol blocks).
        State,
        /// `->` — state transition arrow (session types).
        Arrow,
    }

    /// Parse a signal type string that may include MEGA-1 extensions.
    ///
    /// Extended syntax: `[qualifiers] base_type [where refinements] [@domain] [#tag] [session ref]`
    ///
    /// Returns the components separately for the caller to assemble into
    /// an `ExtendedType`.
    ///
    /// This is the planned signature; implementation follows in a dedicated PR.
    pub fn parse_extended_type_annotation(
        _input: &str,
    ) -> Result<super::ExtendedType, crate::error::MirrError> {
        // Placeholder: actual implementation will use the extended tokenizer.
        // For now, fall back to the base type parser.
        Err(crate::error::MirrError::ParseError {
            message: "[E100] Extended type parsing not yet implemented.".to_string(),
            span: None,
        })
    }
}

// ===========================================================================
// I) Hardware mapping summary (compile-time vs. synthesis)
// ===========================================================================

/// Documents how each MEGA-1 type feature maps to hardware.
///
/// This is a reference table, not executable code. It is included here
/// alongside the type definitions so that the mapping is co-located with
/// the types it describes.
///
/// | Feature             | Compile-Time | Synthesis Impact                     |
/// |---------------------|--------------|--------------------------------------|
/// | Base `SignalType`    | width check  | Wire width (UInt/SInt in FIRRTL)     |
/// | Refinement types    | range check  | None (wire width from base type)     |
/// | Linear types        | use check    | None (ownership is structural)       |
/// | Effect: pure        | dep check    | Wire only (no flip-flop)             |
/// | Effect: stateful    | dep check    | Register inference (flip-flop)       |
/// | Clock domain        | CDC check    | Clock tree routing                   |
/// | Phantom tags        | tag check    | None (erased before emit)            |
/// | Type-level naturals | dim check    | Array flattening (total wire width)  |
/// | Dependent types     | param check  | Parameterized width/count            |
/// | Session types       | FSM check    | None (protocol is static property)   |
///
/// The general principle: features that constrain _values_ (refinements,
/// phantom tags, session states) are compile-time only. Features that
/// constrain _structure_ (clock domains, effects, array dimensions) may
/// influence synthesis decisions.
pub mod hardware_mapping {
    use super::*;

    /// Determine whether an extended type feature has any impact on
    /// synthesized hardware (as opposed to being purely compile-time).
    pub fn has_synthesis_impact(ty: &ExtendedType) -> bool {
        // Clock domains affect routing
        if ty.clock_domain.is_some() {
            return true;
        }
        // Pure/stateful affects register inference
        if ty.is_pure() || ty.is_stateful() {
            return true;
        }
        // Array dimensions affect total wire width
        if ty.type_nat.is_some() {
            return true;
        }
        // Dependent params may affect width
        if !ty.dependent_params.is_empty() {
            return true;
        }
        // Everything else is compile-time only
        false
    }

    /// Convert an `ExtendedType` to a FIRRTL type string.
    ///
    /// This extends the existing `firrtl_type` function in `emit/firrtl.rs`
    /// to handle array dimensions from type-level naturals.
    pub fn extended_firrtl_type(ty: &ExtendedType) -> String {
        let base = match ty.base {
            SignalType::Bool => "UInt<1>".to_string(),
            SignalType::Unsigned(w) => format!("UInt<{}>", w),
            SignalType::Signed(w) => format!("SInt<{}>", w),
        };

        // If there's a type-level natural, wrap in a FIRRTL vector type
        if let Some(ref nat) = ty.type_nat {
            format!("{}[{}]", base, nat.value)
        } else {
            base
        }
    }
}
