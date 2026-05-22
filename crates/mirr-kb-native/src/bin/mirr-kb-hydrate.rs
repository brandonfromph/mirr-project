use anyhow::Result;
use hf_hub::api::sync::Api;
use std::path::PathBuf;

fn main() -> Result<()> {
    let kb_root = std::env::var("MIRR_KB_ROOT").unwrap_or_else(|_| ".kb".to_string());
    let dest_dir = PathBuf::from(&kb_root).join("models");
    std::fs::create_dir_all(&dest_dir)?;

    println!("Hydrating Nomic V2 weights to {}...", dest_dir.display());

    let api = Api::new()?;
    let repo = api.model("nomic-ai/nomic-embed-text-v2-moe".to_string());

    let model_path = repo.get("model.safetensors")?;
    let tokenizer_path = repo.get("tokenizer.json")?;

    std::fs::copy(model_path, dest_dir.join("model.safetensors"))?;
    std::fs::copy(tokenizer_path, dest_dir.join("tokenizer.json"))?;

    println!("Hydration complete.");
    Ok(())
}
