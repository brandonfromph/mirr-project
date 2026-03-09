// Integration tests for the MIRR "did you mean?" suggestion engine.
//
// These tests exercise the public `closest_match` function which uses
// Levenshtein edit distance to suggest typo corrections for undeclared
// signal and guard references.

use nasa_rust_project::suggest::closest_match;

// ---------------------------------------------------------------------------
// Test: basic typo correction
// ---------------------------------------------------------------------------

#[test]
fn suggest_typo_correction() {
    // "temperture" is missing the 'a' — 1 edit from "temperature".
    let candidates = &["clock", "temperature", "pressure"];
    assert_eq!(closest_match("temperture", candidates), Some("temperature"));
}

// ---------------------------------------------------------------------------
// Test: no match when too different
// ---------------------------------------------------------------------------

#[test]
fn suggest_no_match_when_too_different() {
    // "xyzzy_foo_bar" is far from all candidates (well over 3 edits).
    let candidates = &["clock", "temperature", "pressure"];
    assert_eq!(closest_match("xyzzy_foo_bar", candidates), None);
}

// ---------------------------------------------------------------------------
// Test: empty candidates
// ---------------------------------------------------------------------------

#[test]
fn suggest_empty_candidates() {
    let candidates: &[&str] = &[];
    assert_eq!(closest_match("anything", candidates), None);
}

// ---------------------------------------------------------------------------
// Test: exact match is skipped (not a typo)
// ---------------------------------------------------------------------------

#[test]
fn suggest_skips_exact_match() {
    // If the name IS in candidates, skip it — it is not a typo.
    let candidates = &["clock", "temperature"];
    assert_eq!(closest_match("clock", candidates), None);
}

// ---------------------------------------------------------------------------
// Test: single character edit (deletion)
// ---------------------------------------------------------------------------

#[test]
fn suggest_single_char_edit() {
    // "clk_" is 1 deletion away from "clk".
    let candidates = &["clk", "rst", "clr"];
    assert_eq!(closest_match("clk_", candidates), Some("clk"));
}

// ---------------------------------------------------------------------------
// Test: picks the closest among multiple candidates
// ---------------------------------------------------------------------------

#[test]
fn suggest_picks_closest() {
    // "tempurature" differs from "temperature" by 1 substitution (u->e at pos 4).
    // "pressure" is many edits away. Temperature should be picked.
    let candidates = &["pressure", "temperature"];
    assert_eq!(closest_match("tempurature", candidates), Some("temperature"));
}
