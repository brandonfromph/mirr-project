#![forbid(unsafe_code)]

//! Hardware-preparatory pattern matching over tagged words.
//!
//! Implements a fixed-priority pattern match unit for the R-SPU ISA.
//! The match engine scans patterns in priority order (first match wins),
//! bounded by `MAX_MATCH_PATTERNS` (NASA Power-of-10).

use crate::emit::rspu_tagged::{TaggedWord, TypeTag};

/// Maximum patterns per match table (hardware match unit bound).
pub const MAX_MATCH_PATTERNS: usize = 16;

/// A single pattern entry for the hardware match unit.
///
/// Matching: `(word.tag_byte & tag_mask) == tag_pattern &&
///            (word.value & value_mask) == value_pattern`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPattern {
    /// Bitmask for tag comparison (which tag bits to check).
    pub tag_mask: u8,
    /// Expected tag pattern (after masking).
    pub tag_pattern: u8,
    /// Bitmask for value comparison (which value bits to check).
    pub value_mask: u64,
    /// Expected value pattern (after masking).
    pub value_pattern: u64,
    /// Action to take on match.
    pub action: MatchAction,
}

/// Action taken when a pattern matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchAction {
    /// Accept with a pattern ID (for downstream logic).
    Accept(u32),
    /// Raise exception with given code.
    Trap(u8),
    /// Skip -- try next pattern.
    Continue,
}

/// Encode a `TypeTag` as a byte for pattern matching.
///
/// | Tag            | Byte |
/// |----------------|------|
/// | Bool           |    0 |
/// | Unsigned       |    1 |
/// | Signed         |    2 |
/// | Uninitialized  |    3 |
/// | Interval       |    4 |
fn tag_to_byte(word: &TaggedWord) -> u8 {
    match word.tag {
        TypeTag::Bool => 0,
        TypeTag::Unsigned { .. } => 1,
        TypeTag::Signed { .. } => 2,
        TypeTag::Uninitialized => 3,
        TypeTag::Interval { .. } => 4,
    }
}

/// Match a tagged word against a pattern table.
///
/// Fixed-priority scan: iterates patterns in order, returns the action of the
/// first match. Returns `MatchAction::Continue` if no pattern matches.
///
/// Bounded: at most `MAX_MATCH_PATTERNS` iterations (NASA P10).
pub fn match_word(word: &TaggedWord, patterns: &[MatchPattern]) -> MatchAction {
    let limit = patterns.len().min(MAX_MATCH_PATTERNS);
    // Bounded: at most MAX_MATCH_PATTERNS (16) iterations.
    for pat in patterns.iter().take(limit) {
        let tag_byte = tag_to_byte(word);
        if (tag_byte & pat.tag_mask) == pat.tag_pattern
            && (word.value & pat.value_mask) == pat.value_pattern
        {
            return pat.action;
        }
    }
    MatchAction::Continue
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::rspu_tagged::Provenance;

    /// Helper: create a simple unsigned tagged word for testing.
    fn unsigned_word(value: u64, width: u8) -> TaggedWord {
        TaggedWord { value, tag: TypeTag::Unsigned { width }, provenance: Provenance::Literal }
    }

    /// Helper: create a bool tagged word for testing.
    fn bool_word(value: u64) -> TaggedWord {
        TaggedWord { value, tag: TypeTag::Bool, provenance: Provenance::Literal }
    }

    #[test]
    fn test_tag_to_byte_all_variants() {
        let w_bool = bool_word(1);
        assert_eq!(tag_to_byte(&w_bool), 0);

        let w_unsigned = unsigned_word(42, 8);
        assert_eq!(tag_to_byte(&w_unsigned), 1);

        let w_signed = TaggedWord {
            value: 0,
            tag: TypeTag::Signed { width: 16 },
            provenance: Provenance::Literal,
        };
        assert_eq!(tag_to_byte(&w_signed), 2);

        let w_uninit = TaggedWord::uninitialized();
        assert_eq!(tag_to_byte(&w_uninit), 3);

        let w_interval = TaggedWord {
            value: 50,
            tag: TypeTag::Interval { lo: 0, hi: 100 },
            provenance: Provenance::Computed,
        };
        assert_eq!(tag_to_byte(&w_interval), 4);
    }

    #[test]
    fn test_match_word_first_match_wins() {
        let word = unsigned_word(0xAB, 8);
        let patterns = [
            MatchPattern {
                tag_mask: 0xFF,
                tag_pattern: 0, // Bool tag -- won't match
                value_mask: 0xFF,
                value_pattern: 0xAB,
                action: MatchAction::Trap(1),
            },
            MatchPattern {
                tag_mask: 0xFF,
                tag_pattern: 1, // Unsigned tag -- matches
                value_mask: 0xFF,
                value_pattern: 0xAB,
                action: MatchAction::Accept(42),
            },
            MatchPattern {
                tag_mask: 0xFF,
                tag_pattern: 1, // Also matches but lower priority
                value_mask: 0xFF,
                value_pattern: 0xAB,
                action: MatchAction::Accept(99),
            },
        ];
        assert_eq!(match_word(&word, &patterns), MatchAction::Accept(42));
    }

    #[test]
    fn test_match_word_no_match_returns_continue() {
        let word = bool_word(1);
        let patterns = [MatchPattern {
            tag_mask: 0xFF,
            tag_pattern: 1, // Unsigned -- won't match Bool
            value_mask: 0xFF,
            value_pattern: 1,
            action: MatchAction::Accept(0),
        }];
        assert_eq!(match_word(&word, &patterns), MatchAction::Continue);
    }

    #[test]
    fn test_match_word_empty_patterns() {
        let word = unsigned_word(0, 8);
        assert_eq!(match_word(&word, &[]), MatchAction::Continue);
    }

    #[test]
    fn test_match_word_wildcard_tag() {
        // tag_mask = 0 means "match any tag"
        let word = unsigned_word(0x42, 16);
        let patterns = [MatchPattern {
            tag_mask: 0,
            tag_pattern: 0,
            value_mask: 0xFF,
            value_pattern: 0x42,
            action: MatchAction::Accept(1),
        }];
        assert_eq!(match_word(&word, &patterns), MatchAction::Accept(1));
    }

    #[test]
    fn test_match_word_value_mask() {
        // Only check low nibble of value
        let word = unsigned_word(0xF5, 8);
        let patterns = [MatchPattern {
            tag_mask: 0xFF,
            tag_pattern: 1,
            value_mask: 0x0F,
            value_pattern: 0x05,
            action: MatchAction::Trap(7),
        }];
        assert_eq!(match_word(&word, &patterns), MatchAction::Trap(7));
    }

    #[test]
    fn test_match_word_respects_max_bound() {
        // Create more than MAX_MATCH_PATTERNS entries; only first 16 checked.
        let mut patterns = Vec::with_capacity(MAX_MATCH_PATTERNS + 2);
        // Bounded: MAX_MATCH_PATTERNS + 2 iterations.
        for _i in 0..(MAX_MATCH_PATTERNS + 2) {
            patterns.push(MatchPattern {
                tag_mask: 0xFF,
                tag_pattern: 0xFF, // Won't match anything
                value_mask: 0,
                value_pattern: 0,
                action: MatchAction::Trap(0),
            });
        }
        // Put a matching pattern beyond the bound.
        patterns[MAX_MATCH_PATTERNS].tag_pattern = 1;
        patterns[MAX_MATCH_PATTERNS].tag_mask = 0xFF;
        patterns[MAX_MATCH_PATTERNS].action = MatchAction::Accept(999);

        let word = unsigned_word(0, 8);
        // Pattern at index 16 is beyond the bound, so it should not be reached.
        assert_eq!(match_word(&word, &patterns), MatchAction::Continue);
    }
}
