//! Source position tracking for MIRR compiler diagnostics.
//!
//! Provides the [`Span`] type used throughout the AST, error types, and
//! LSP server to map compiler entities back to source locations.
//!
//! [`FileTable`] provides string-interned file path storage so that
//! each `Span` only carries a lightweight `u32` index instead of a
//! heap-allocated `String` for the source file path.
//!
//! Line and column numbers are 0-based to match the LSP protocol.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Maximum number of unique source files tracked by the compiler.
/// Bounded per NASA Power-of-10 Rule #2.
const MAX_SOURCE_FILES: usize = 4096;

/// String-interned table of source file paths.
///
/// During parsing, each unique file path is registered once via [`intern`].
/// The returned `u32` index is stored in [`Span::file_id`], avoiding
/// per-node heap allocation of the full path string.
#[derive(Debug, Clone, Default)]
pub struct FileTable {
    paths: Vec<String>,
}

impl FileTable {
    /// Create an empty file table.
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Intern a file path, returning its index.
    ///
    /// If the path was already interned, returns the existing index.
    /// Bounded to [`MAX_SOURCE_FILES`] entries.
    pub fn intern(&mut self, path: &str) -> u32 {
        // Check for existing entry first (bounded linear scan).
        for (i, existing) in self.paths.iter().enumerate().take(MAX_SOURCE_FILES) {
            if existing == path {
                return i as u32;
            }
        }
        // Insert new entry.
        let id = self.paths.len() as u32;
        if (id as usize) < MAX_SOURCE_FILES {
            self.paths.push(path.to_string());
        }
        id
    }

    /// Look up a file path by its interned index.
    pub fn get(&self, file_id: u32) -> Option<&str> {
        self.paths.get(file_id as usize).map(|s| s.as_str())
    }

    /// Return the number of interned file paths.
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Return true if no file paths have been interned.
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

/// A source-level span marking a region in the input text.
///
/// Line and column numbers are 0-based to match LSP protocol conventions.
/// `end_col` is exclusive (the character after the last character of the span).
///
/// `file_id` is a string-interned index into a [`FileTable`]. It is `None`
/// for spans created from inline strings (e.g., test fixtures).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Interned file path index (None for inline/string sources).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<u32>,
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
        Self { file_id: None, start_line: line, start_col, end_line: line, end_col }
    }

    /// Create a span covering an entire line (col 0 to end).
    pub fn full_line(line: u32) -> Self {
        Self { file_id: None, start_line: line, start_col: 0, end_line: line, end_col: u32::MAX }
    }

    /// Create a span covering multiple complete lines.
    pub fn multi_line(start_line: u32, end_line: u32) -> Self {
        Self { file_id: None, start_line, start_col: 0, end_line, end_col: u32::MAX }
    }

    /// Create a span with a file path index (from [`FileTable::intern`]).
    pub fn with_file(file_id: u32, start_line: u32, end_line: u32) -> Self {
        Self { file_id: Some(file_id), start_line, start_col: 0, end_line, end_col: u32::MAX }
    }

    /// Format the source location for display (e.g., `"core.mirr:42"`).
    ///
    /// Line numbers are converted from 0-based to 1-based for human display.
    pub fn display_location(&self, table: &FileTable) -> String {
        let file_str = self.file_id.and_then(|id| table.get(id)).unwrap_or("<unknown>");
        // Convert 0-based to 1-based line number for human display.
        format!("{}:{}", file_str, self.start_line + 1)
    }

    /// Merge two spans into one covering both regions.
    ///
    /// Prefers `self.file_id` if both spans have file IDs.
    pub fn merge(self, other: Span) -> Span {
        let file_id = self.file_id.or(other.file_id);
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
        Span { file_id, start_line, start_col, end_line, end_col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_span() {
        let s = Span::single_line(5, 10, 20);
        assert_eq!(s.file_id, None);
        assert_eq!(s.start_line, 5);
        assert_eq!(s.start_col, 10);
        assert_eq!(s.end_line, 5);
        assert_eq!(s.end_col, 20);
    }

    #[test]
    fn full_line_span() {
        let s = Span::full_line(3);
        assert_eq!(s.file_id, None);
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
        assert_eq!(merged.file_id, None);
        assert_eq!(merged.start_line, 2);
        assert_eq!(merged.start_col, 5);
        assert_eq!(merged.end_line, 4);
        assert_eq!(merged.end_col, 15);
    }

    #[test]
    fn file_table_intern_and_lookup() {
        let mut table = FileTable::new();
        let id0 = table.intern("core/core_top.mirr");
        let id1 = table.intern("interconnect/noc_router.mirr");
        let id0_dup = table.intern("core/core_top.mirr");

        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(id0_dup, 0, "duplicate path must return existing ID");
        assert_eq!(table.len(), 2);
        assert_eq!(table.get(id0), Some("core/core_top.mirr"));
        assert_eq!(table.get(id1), Some("interconnect/noc_router.mirr"));
        assert_eq!(table.get(99), None);
    }

    #[test]
    fn span_with_file_and_display() {
        let mut table = FileTable::new();
        let fid = table.intern("rspu_top.mirr");
        let s = Span::with_file(fid, 41, 41);
        assert_eq!(s.file_id, Some(0));
        // 0-based line 41 displays as 1-based line 42
        assert_eq!(s.display_location(&table), "rspu_top.mirr:42");
    }

    #[test]
    fn span_display_location_without_file() {
        let table = FileTable::new();
        let s = Span::full_line(9);
        assert_eq!(s.display_location(&table), "<unknown>:10");
    }

    #[test]
    fn merge_preserves_file_id() {
        let mut table = FileTable::new();
        let fid = table.intern("test.mirr");
        let a = Span::with_file(fid, 2, 3);
        let b = Span::full_line(5);
        let merged = a.merge(b);
        assert_eq!(merged.file_id, Some(fid), "merge must preserve file_id from self");
    }
}
