//! Source position tracking for MIRR compiler diagnostics.
//!
//! Provides the [`Span`] type used throughout the AST, error types, and
//! LSP server to map compiler entities back to source locations.
//!
//! Line and column numbers are 0-based to match the LSP protocol.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// A source-level span marking a region in the input text.
///
/// Line and column numbers are 0-based to match LSP protocol conventions.
/// `end_col` is exclusive (the character after the last character of the span).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Start line (0-based).
    pub start_line: u32,
    /// Start column / character offset within the line (0-based).
    pub start_col: u32,
    /// End line (0-based).
    pub end_line: u32,
    /// End column / character offset (exclusive, 0-based).
    pub end_col: u32,
}

impl Span {
    /// Create a span covering a portion of a single line.
    pub fn single_line(line: u32, start_col: u32, end_col: u32) -> Self {
        Self { start_line: line, start_col, end_line: line, end_col }
    }

    /// Create a span covering an entire line (col 0 to end).
    pub fn full_line(line: u32) -> Self {
        Self { start_line: line, start_col: 0, end_line: line, end_col: u32::MAX }
    }

    /// Create a span covering multiple complete lines.
    pub fn multi_line(start_line: u32, end_line: u32) -> Self {
        Self { start_line, start_col: 0, end_line, end_col: u32::MAX }
    }

    /// Merge two spans into one covering both regions.
    pub fn merge(self, other: Span) -> Span {
        let (start_line, start_col) = if self.start_line < other.start_line
            || (self.start_line == other.start_line && self.start_col <= other.start_col)
        {
            (self.start_line, self.start_col)
        } else {
            (other.start_line, other.start_col)
        };
        let (end_line, end_col) = if self.end_line > other.end_line
            || (self.end_line == other.end_line && self.end_col >= other.end_col)
        {
            (self.end_line, self.end_col)
        } else {
            (other.end_line, other.end_col)
        };
        Span { start_line, start_col, end_line, end_col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_span() {
        let s = Span::single_line(5, 10, 20);
        assert_eq!(s.start_line, 5);
        assert_eq!(s.start_col, 10);
        assert_eq!(s.end_line, 5);
        assert_eq!(s.end_col, 20);
    }

    #[test]
    fn full_line_span() {
        let s = Span::full_line(3);
        assert_eq!(s.start_line, 3);
        assert_eq!(s.start_col, 0);
        assert_eq!(s.end_line, 3);
        assert_eq!(s.end_col, u32::MAX);
    }

    #[test]
    fn merge_spans() {
        let a = Span::single_line(2, 5, 10);
        let b = Span::single_line(4, 0, 15);
        let merged = a.merge(b);
        assert_eq!(merged.start_line, 2);
        assert_eq!(merged.start_col, 5);
        assert_eq!(merged.end_line, 4);
        assert_eq!(merged.end_col, 15);
    }
}
