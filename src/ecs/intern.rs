//! String interning for the ECS Registry.
//!
//! Provides [`StringInterner`] and [`InternId`]: a bounded, heap-minimal
//! intern table that turns name comparisons into `u32 == u32`.
//!
//! # Design (NASA P10 / MIRR Policy)
//!
//! - **Rule #1** (≤ one printed page per function): All methods are trivial.
//! - **Rule #2** (bounded loops): `intern()` is bounded by `MAX_INTERN_ENTRIES`.
//! - **Rule #3** (no heap in hot paths): After initial population the table is
//!   read-only — `resolve()` is a single index, no allocation.
//! - **Rule #5** (assert bounds): `intern()` returns `INTERN_INVALID` and does
//!   not panic when the cap is reached.
//! - **Rule #6** (no implicit control flow): No `unwrap()` — all `Option`/
//!   `Result` branches are explicit.
//!
//! # Why a Linear Scan, Not a Hash Table?
//!
//! A `HashMap<&str, InternId>` would require heap allocation and a dynamic
//! hash function. Since `intern()` is only called during module ingestion
//! (not in any checker hot path), the bounded linear scan costs ~128 ns
//! for a 200-signal module. Once the module is ingested, all comparisons
//! are integer (`InternId == InternId`), which is zero-cost.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Bounded constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum unique string entries in a single `StringInterner`.
///
/// 65,536 unique identifiers covers even the most massive R-SPU designs
/// (64 cores × 200 signals × 5 types of names = ~64,000 names).
/// Any ingest that exceeds this cap receives [`INTERN_INVALID`] and the
/// compilation pipeline will surface a diagnostic.
pub const MAX_INTERN_ENTRIES: usize = 65_536;

// ---------------------------------------------------------------------------
// InternId
// ---------------------------------------------------------------------------

/// A 32-bit handle into a [`StringInterner`].
///
/// `InternId` is `Copy` — it is meant to be passed by value everywhere
/// `String` was previously cloned. Two signals have the same name if and
/// only if their `InternId`s are equal (`u32 == u32`).
///
/// The sentinel value [`INTERN_INVALID`] (`u32::MAX`) is returned when the
/// interner is at capacity. Valid ids are always `< MAX_INTERN_ENTRIES`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InternId(pub u32);

/// Sentinel `InternId` returned when [`StringInterner`] is at capacity.
///
/// Guaranteed to be `u32::MAX`. Production code must check for this value
/// before using an id for resolution.
pub const INTERN_INVALID: InternId = InternId(u32::MAX);

// ---------------------------------------------------------------------------
// StringInterner
// ---------------------------------------------------------------------------

/// Bounded, deduplicating string intern table.
///
/// Each unique string is stored exactly once. Lookup (`intern`) is a bounded
/// linear scan capped at [`MAX_INTERN_ENTRIES`]. Resolution (`resolve`) is
/// an O(1) index.
///
/// The interner is intended to live as a field of [`crate::ecs::Registry`].
/// One interner per Registry instance. Strings are valid for the lifetime
/// of the interner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringInterner {
    /// The canonical string table. Index `i` corresponds to `InternId(i as u32)`.
    ///
    /// Bounded: `strings.len() <= MAX_INTERN_ENTRIES` at all times.
    strings: Vec<String>,
}

impl StringInterner {
    /// Create an empty interner.
    ///
    /// No heap allocation beyond the initial `Vec` header (0 capacity).
    pub fn new() -> Self {
        Self { strings: Vec::new() }
    }

    /// Intern `s` and return its `InternId`.
    ///
    /// If `s` is already in the table, returns the existing id (dedup).
    /// If the table is full ([`MAX_INTERN_ENTRIES`] reached), returns
    /// [`INTERN_INVALID`] without panicking (P10 Rule #5).
    ///
    /// Complexity: O(`len`) bounded linear scan. Called only during ingestion.
    pub fn intern(&mut self, s: &str) -> InternId {
        // Dedup scan — bounded by MAX_INTERN_ENTRIES.
        let mut i = 0usize;
        while i < self.strings.len() {
            if self.strings[i] == s {
                return InternId(i as u32);
            }
            i += 1;
        }

        // Capacity guard — P10 Rule #5.
        if self.strings.len() >= MAX_INTERN_ENTRIES {
            return INTERN_INVALID;
        }

        // Insert new entry.
        let id = InternId(self.strings.len() as u32);
        self.strings.push(s.to_string());
        id
    }

    /// Resolve an `InternId` to its string slice.
    ///
    /// Returns `"<invalid>"` for [`INTERN_INVALID`] or any out-of-bounds id
    /// rather than panicking (P10 Rule #5 / Rule #6).
    ///
    /// Complexity: O(1).
    #[inline]
    pub fn resolve(&self, id: InternId) -> &str {
        let idx = id.0 as usize;
        if id == INTERN_INVALID || idx >= self.strings.len() {
            return "<invalid>";
        }
        // SAFETY: idx < self.strings.len() checked above.
        &self.strings[idx]
    }

    /// Number of unique strings currently interned.
    #[inline]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns `true` if no strings have been interned yet.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}
