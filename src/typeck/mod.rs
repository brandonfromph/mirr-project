//! ARCHITECTURAL SUB-ENGINE: SEMANTIC TYPE CHECKER
//!
//! TYPE-001/TYPE-002/TYPE-003: Semantic type checker for MIRR modules.
//!
//! Runs after semantic validation (name/reference checks) and before
//! simplification. Enforces type compatibility across all expressions:
//! guard conditions, reflex assignments, and property formulas.
//!
//! Type rules are documented in `proposals/002-TYPE-001-2026-03-08.md`
//! and `proposals/003-TYPE-002-2026-03-08.md`.
//!
//! Error codes: E601–E607 (see `docs/error_codes.md`).
//!
//! ## MEGA-1 Extended Type System
//!
//! The `extended` submodule adds refinement types, linear types, effect types,
//! clock domain qualifiers, phantom types, type-level naturals, dependent types,
//! and session types. See `typeck::extended` for details. Error codes: E610–E625.

#![forbid(unsafe_code)]

pub mod extended;
