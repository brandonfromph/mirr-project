#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

const MAX_CHUNK_SIZE_CHARS: usize = 8192;

/// Semantic chunk type in MIRR source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChunkType {
    /// Entire module declaration (parent chunk).
    Module,
    /// Signal declaration with type and initialization.
    Signal,
    /// Guard condition expression.
    Guard,
    /// Reflex body with assignments.
    Reflex,
    /// Property formula (safety obligation).
    Property,
}

/// A semantic chunk at MIRR-native boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrChunk {
    /// Unique chunk identifier: e.g., "module_foo.signal_bar" or "module_foo.reflex_r1"
    pub id: String,

    /// Type of chunk (module, signal, guard, reflex, property).
    pub chunk_type: ChunkType,

    /// Source text of the chunk (the actual code).
    pub text: String,

    /// Parent module name.
    pub module: String,

    /// Parent identifier if nested (e.g., signal_id for a reflex inside that signal).
    pub parent_id: Option<String>,

    /// Source line range (start, end) in original file.
    pub line_range: (usize, usize),

    /// Hash of text for incremental indexing and change detection.
    pub hash: String,

    /// Estimated token count (conservative: len / 4 + 1).
    pub estimated_tokens: usize,
}

impl MirrChunk {
    /// Create a new MIRR chunk.
    pub fn new(
        id: String,
        chunk_type: ChunkType,
        text: String,
        module: String,
        parent_id: Option<String>,
        line_range: (usize, usize),
    ) -> Self {
        let hash = compute_hash(&text);
        let estimated_tokens = estimate_token_count(&text);

        Self { id, chunk_type, text, module, parent_id, line_range, hash, estimated_tokens }
    }

    /// Check if this chunk would exceed size constraints.
    pub fn exceeds_size_limit(&self) -> bool {
        self.text.len() > MAX_CHUNK_SIZE_CHARS
    }
}

/// Hash the chunk text for incremental indexing.
pub fn compute_hash(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

/// Estimate token count conservatively (1 token ≈ 4 characters).
pub fn estimate_token_count(text: &str) -> usize {
    text.len() / 4 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_creation_computes_hash() {
        let chunk = MirrChunk::new(
            "test.signal_x".to_string(),
            ChunkType::Signal,
            "signal x: u8 = 0;".to_string(),
            "test".to_string(),
            None,
            (1, 1),
        );
        assert!(!chunk.hash.is_empty());
        assert_eq!(chunk.hash.len(), 64); // SHA256 hex is 64 chars
    }

    #[test]
    fn chunk_creation_estimates_tokens() {
        let chunk = MirrChunk::new(
            "test.signal_x".to_string(),
            ChunkType::Signal,
            "x".to_string(), // 1 char ≈ 1 token
            "test".to_string(),
            None,
            (1, 1),
        );
        assert!(chunk.estimated_tokens >= 1);
    }

    #[test]
    fn large_chunk_detected() {
        let large_text = "a".repeat(MAX_CHUNK_SIZE_CHARS + 1);
        let chunk = MirrChunk::new(
            "test.large".to_string(),
            ChunkType::Module,
            large_text,
            "test".to_string(),
            None,
            (1, 100),
        );
        assert!(chunk.exceeds_size_limit());
    }
}
