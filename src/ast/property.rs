// ---------------------------------------------------------------------------
//! Safety property declarations for formal verification.
//!
//! Properties compile to SystemVerilog Assertions (SVA) and do not affect
//! generated hardware.
//!
//! ## Formula variants (6)
//!
//! | Variant | MIRR syntax | SVA output |
//! |---------|-------------|------------|
//! | Always | `always (P)` | `P` |
//! | Never | `never (P)` | `!(P)` |
//! | AlwaysImplies | `always (P -> Q)` | `P \|-> Q` |
//! | NeverImplies | `never (P -> Q)` | `!(P \|-> Q)` |
//! | EventuallyWithin | `eventually within N (P)` | `##[1:N] P` |
//! | AlwaysFollowedBy | `always (P followed_by N Q)` | `P \|-> ##N Q` |
//!
//! ## Directives (3)
//!
//! | Directive | SVA keyword |
//! |-----------|-------------|
//! | Assert | `assert property` |
//! | Cover | `cover property` |
//! | Assume | `assume property` |
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use super::expr::Expr;

/// Verification directive controlling the SVA wrapper keyword.
///
/// Defaults to `Assert` for backward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PropertyDirective {
    /// `assert property` — P must hold (default).
    #[default]
    Assert,
    /// `cover property` — prove P is reachable.
    Cover,
    /// `assume property` — constrain formal verification inputs.
    Assume,
}

/// The formula inside a property declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyFormula {
    /// `always (P)` — P must hold every cycle.
    Always(Expr),
    /// `never (P)` — P must never hold.
    Never(Expr),
    /// `always (P -> Q)` — whenever P holds, Q must also hold (same cycle).
    AlwaysImplies { antecedent: Expr, consequent: Expr },
    /// `never (P -> Q)` — it must never be the case that P implies Q.
    NeverImplies { antecedent: Expr, consequent: Expr },
    /// `eventually within N (P)` — P must become true within N cycles.
    EventuallyWithin { expr: Expr, cycles: u32 },
    /// `always (P followed_by N Q)` — whenever P holds, Q must hold N cycles later.
    AlwaysFollowedBy { trigger: Expr, response: Expr, delay_cycles: u32 },
}

impl PropertyFormula {
    /// Collect references to all expressions in this formula.
    pub fn exprs(&self) -> Vec<&Expr> {
        match self {
            Self::Always(e) | Self::Never(e) | Self::EventuallyWithin { expr: e, .. } => vec![e],
            Self::AlwaysImplies { antecedent, consequent }
            | Self::NeverImplies { antecedent, consequent } => vec![antecedent, consequent],
            Self::AlwaysFollowedBy { trigger, response, .. } => vec![trigger, response],
        }
    }

    /// Collect mutable references to all expressions in this formula.
    pub fn exprs_mut(&mut self) -> Vec<&mut Expr> {
        match self {
            Self::Always(e) | Self::Never(e) | Self::EventuallyWithin { expr: e, .. } => vec![e],
            Self::AlwaysImplies { antecedent, consequent }
            | Self::NeverImplies { antecedent, consequent } => vec![antecedent, consequent],
            Self::AlwaysFollowedBy { trigger, response, .. } => vec![trigger, response],
        }
    }
}

/// A named safety property declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyDecl {
    pub name: String,
    /// Verification directive (assert/cover/assume). Defaults to Assert.
    #[serde(default)]
    pub directive: PropertyDirective,
    pub formula: PropertyFormula,
    /// Pattern origin tag for DO-178C traceability (`None` for hand-written properties).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
