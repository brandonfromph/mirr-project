#![forbid(unsafe_code)]

const MAX_CONTEXT_TOKENS: usize = 8000;
const MAX_CHUNKS_IN_CONTEXT: usize = 8;
const MAX_QUERY_BYTES: usize = 4096;
const CHARS_PER_TOKEN: usize = 4; // Conservative estimate

/// Enforces hard limits on context assembly for RAG queries.
#[derive(Debug, Clone)]
pub struct ContextBudget {
    available_tokens: usize,
    available_chunks: usize,
}

impl Default for ContextBudget {
    fn default() -> Self {
        Self { available_tokens: MAX_CONTEXT_TOKENS, available_chunks: MAX_CHUNKS_IN_CONTEXT }
    }
}

impl ContextBudget {
    /// Create a new context budget with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempt to add a chunk to the context.
    /// Returns true if the chunk fits within remaining budget.
    pub fn try_add_chunk(&mut self, chunk_text: &str) -> bool {
        let chunk_tokens = estimate_token_count(chunk_text);

        // Check both constraints
        if self.available_tokens >= chunk_tokens && self.available_chunks > 0 {
            self.available_tokens -= chunk_tokens;
            self.available_chunks -= 1;
            true
        } else {
            false
        }
    }

    /// Get remaining budget (tokens, chunks).
    pub fn remaining(&self) -> (usize, usize) {
        (self.available_tokens, self.available_chunks)
    }

    /// Check if budget is exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.available_tokens == 0 || self.available_chunks == 0
    }

    /// Reset budget to defaults.
    pub fn reset(&mut self) {
        self.available_tokens = MAX_CONTEXT_TOKENS;
        self.available_chunks = MAX_CHUNKS_IN_CONTEXT;
    }
}

/// Estimate token count conservatively: 1 token ≈ 4 characters.
pub fn estimate_token_count(text: &str) -> usize {
    (text.len() + CHARS_PER_TOKEN - 1) / CHARS_PER_TOKEN
}

/// Validate query size.
pub fn validate_query_size(query: &str) -> Result<(), String> {
    if query.is_empty() {
        return Err("Query cannot be empty".to_string());
    }
    if query.len() > MAX_QUERY_BYTES {
        return Err(format!(
            "Query exceeds max size: {} bytes > {} bytes",
            query.len(),
            MAX_QUERY_BYTES
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_tracks_tokens() {
        let mut budget = ContextBudget::new();
        assert_eq!(budget.remaining(), (MAX_CONTEXT_TOKENS, MAX_CHUNKS_IN_CONTEXT));

        let chunk = "a".repeat(100); // ~25 tokens
        assert!(budget.try_add_chunk(&chunk));
        let (tokens, chunks) = budget.remaining();
        assert!(tokens < MAX_CONTEXT_TOKENS);
        assert_eq!(chunks, MAX_CHUNKS_IN_CONTEXT - 1);
    }

    #[test]
    fn budget_enforces_chunk_limit() {
        let mut budget = ContextBudget::new();
        for i in 0..MAX_CHUNKS_IN_CONTEXT {
            let chunk = format!("chunk_{}", i);
            assert!(budget.try_add_chunk(&chunk), "chunk {} should fit", i);
        }

        // Next chunk should be rejected
        assert!(!budget.try_add_chunk("extra chunk"));
    }

    #[test]
    fn budget_enforces_token_limit() {
        let mut budget = ContextBudget::new();
        let huge_chunk = "a".repeat(MAX_CONTEXT_TOKENS * CHARS_PER_TOKEN + 100);
        assert!(!budget.try_add_chunk(&huge_chunk));
    }

    #[test]
    fn token_estimation_is_conservative() {
        let text = "Hello, world!"; // 13 chars
        let tokens = estimate_token_count(text);
        assert!(tokens >= 3); // At least 13/4 = 3
        assert!(tokens <= 4); // At most (13+3)/4 = 4
    }

    #[test]
    fn query_validation_rejects_empty() {
        assert!(validate_query_size("").is_err());
    }

    #[test]
    fn query_validation_rejects_oversized() {
        let huge_query = "a".repeat(MAX_QUERY_BYTES + 1);
        assert!(validate_query_size(&huge_query).is_err());
    }

    #[test]
    fn query_validation_accepts_valid() {
        assert!(validate_query_size("what signals does module X emit").is_ok());
    }
}
