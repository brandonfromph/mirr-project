//! "Did you mean?" suggestions via Levenshtein edit distance.
//!
//! Used by semantic validation to suggest fixes for undeclared signal
//! and guard references.

#![forbid(unsafe_code)]

/// Maximum candidates to evaluate (bounded iteration — NASA P10).
const MAX_CANDIDATES: usize = 1024;

/// Maximum edit distance to consider a suggestion relevant.
const MAX_EDIT_DISTANCE: usize = 3;

/// Find the closest match to `name` among `candidates`.
///
/// Returns `None` if no candidate is within `MAX_EDIT_DISTANCE` edits,
/// or if `candidates` is empty.
pub fn closest_match<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let mut best: Option<&'a str> = None;
    let mut best_dist = MAX_EDIT_DISTANCE + 1;

    // Bounded: process at most MAX_CANDIDATES entries.
    let limit = candidates.len().min(MAX_CANDIDATES);
    for &candidate in &candidates[..limit] {
        // Skip identical matches.
        if candidate == name {
            continue;
        }
        let dist = levenshtein(name, candidate);
        if dist < best_dist {
            best_dist = dist;
            best = Some(candidate);
        }
    }
    best
}

/// Compute the Levenshtein edit distance between two strings.
///
/// Uses a single-row dynamic programming approach: O(n*m) time,
/// O(min(n,m)) space. Purely iterative — no recursion (NASA P10).
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (n, m) = (a_chars.len(), b_chars.len());

    // Ensure we iterate over the shorter dimension for the row.
    let (long, short) = if n >= m { (&a_chars, &b_chars) } else { (&b_chars, &a_chars) };
    let (long_len, short_len) = (long.len(), short.len());

    // Single-row DP: row[j] = edit distance for first i chars of long
    // vs first j chars of short.
    let mut row: Vec<usize> = (0..=short_len).collect();

    for i in 1..=long_len {
        let mut prev = row[0];
        row[0] = i;
        for j in 1..=short_len {
            let old = row[j];
            let cost = if long[i - 1] == short[j - 1] { 0 } else { 1 };
            row[j] = (prev + cost)
                .min(row[j] + 1) // deletion
                .min(row[j - 1] + 1); // insertion
            prev = old;
        }
    }
    row[short_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Levenshtein distance unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("xyz", ""), 3);
    }

    #[test]
    fn test_levenshtein_single_edit() {
        // Substitution: kitten -> sitten (k -> s).
        assert_eq!(levenshtein("kitten", "sitten"), 1);
    }

    #[test]
    fn test_levenshtein_multiple() {
        // kitten -> sitting requires 3 edits:
        //   k->s, e->i, +g
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }

    // -----------------------------------------------------------------------
    // closest_match unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_closest_match_found() {
        let candidates = &["clock", "reset", "enable", "data_in"];
        // "clok" is 1 edit away from "clock".
        assert_eq!(closest_match("clok", candidates), Some("clock"));
    }

    #[test]
    fn test_closest_match_too_far() {
        let candidates = &["clock", "reset", "enable"];
        // "xyzzy" is far from all candidates (> MAX_EDIT_DISTANCE).
        assert_eq!(closest_match("xyzzy", candidates), None);
    }

    #[test]
    fn test_closest_match_empty() {
        let candidates: &[&str] = &[];
        assert_eq!(closest_match("anything", candidates), None);
    }

    #[test]
    fn test_closest_match_skips_identical() {
        let candidates = &["clock", "reset"];
        // Exact match "clock" should be skipped — no suggestion needed
        // when the name already exists.
        assert_eq!(closest_match("clock", candidates), None);
    }
}
