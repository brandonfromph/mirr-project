#![forbid(unsafe_code)]
#![deny(warnings)]

use async_trait::async_trait;

/// Trait for embedding providers.
/// Implementations: Gemini, Local deterministic embeddings, etc.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Generate embedding vector for text.
    /// Returns a Vec of floats representing the embedding.
    async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>>;
}

/// Stub implementation for Phase 2 integration.
/// Will be replaced with actual Gemini client when credentials are available.
pub struct StubEmbeddingProvider;

#[async_trait]
impl EmbeddingProvider for StubEmbeddingProvider {
    async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
        Err(anyhow::anyhow!("Embedding provider not configured; use deterministic fallback"))
    }
}
