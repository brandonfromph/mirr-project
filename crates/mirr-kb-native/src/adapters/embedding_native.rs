#![allow(unsafe_code)]

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;

/// Native inference provider using HuggingFace's Candle library.
pub struct NativeEmbeddingProvider {
    device: Device,
    word_embeddings: Tensor,
    tokenizer: Tokenizer,
}

impl NativeEmbeddingProvider {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        #[cfg(target_os = "macos")]
        let device = Device::new_metal(0).unwrap_or(Device::Cpu);
        #[cfg(not(target_os = "macos"))]
        let device = Device::Cpu;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!(format!("Tokenizer load error: {}", e)))?;

        // Load model weights (assuming safetensors format)
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, &device)?
        };

        let word_embeddings = vb.get((250048, 768), "embeddings.word_embeddings.weight")?;

        Ok(Self { device, word_embeddings, tokenizer })
    }
}

#[async_trait]
impl crate::adapters::embedding::EmbeddingProvider for NativeEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let tokens = self.tokenizer.encode(text, true).map_err(|e| anyhow!(e))?;
        let token_ids = tokens.get_ids();

        let ids = Tensor::new(token_ids, &self.device)?.unsqueeze(0)?;
        let embeddings = self.word_embeddings.embedding(&ids)?;

        // Mean pooling
        let pooled = embeddings.mean(1)?;
        let vec: Vec<f32> = pooled.flatten_all()?.to_vec1::<f32>()?;

        // Normalize vector
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        let normalized: Vec<f32> = vec.iter().map(|x| x / norm).collect();

        Ok(normalized)
    }
}
