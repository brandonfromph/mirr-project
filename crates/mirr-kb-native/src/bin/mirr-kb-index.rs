use anyhow::Result;
use clap::{CommandFactory, Parser};
use mirr_kb_native::adapters::embedding::EmbeddingProvider;
use mirr_kb_native::adapters::embedding_native::NativeEmbeddingProvider;
use mirr_kb_native::chunking::{ChunkType, MirrChunk};
use mirr_kb_native::storage::SqliteHybridStorage;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "mirr-kb-index", version, about = "MIRR Knowledge Base Indexer")]
struct Args {
    /// Path to scan for MIRR files
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Root directory for KB assets
    #[arg(long, default_value = ".kb")]
    kb_root: String,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,

    /// Path to the embedding model
    #[arg(long)]
    model_path: String,

    /// Path to the model tokenizer
    #[arg(long)]
    tokenizer_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.help_json {
        fn get_cmd_manifest(cmd: &clap::Command) -> serde_json::Value {
            let mut args_list = Vec::new();
            for arg in cmd.get_arguments() {
                args_list.push(serde_json::json!({
                    "id": arg.get_id().as_str(),
                    "long": arg.get_long(),
                    "short": arg.get_short(),
                    "help": arg.get_help().map(|h| h.to_string()),
                    "required": arg.is_required_set(),
                }));
            }
            let mut subs = Vec::new();
            for sub in cmd.get_subcommands() {
                subs.push(get_cmd_manifest(sub));
            }
            serde_json::json!({
                "name": cmd.get_name(),
                "about": cmd.get_about().map(|a| a.to_string()),
                "version": cmd.get_version().map(|v| v.to_string()),
                "args": args_list,
                "subcommands": subs,
            })
        }
        let cmd = Args::command();
        println!("{}", serde_json::to_string_pretty(&get_cmd_manifest(&cmd)).unwrap());
        return Ok(());
    }

    let db_path = PathBuf::from(&args.kb_root).join("graph.db");
    std::fs::create_dir_all(&args.kb_root)?;
    let storage = Arc::new(SqliteHybridStorage::from_db_path(db_path)?);

    let provider = Arc::new(NativeEmbeddingProvider::new(&args.model_path, &args.tokenizer_path)?);

    let files: Vec<PathBuf> = WalkDir::new(args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.extension().map_or(false, |ext| ext == "mirr" || ext == "rs"))
        .collect();

    println!("Parallel indexing {} files...", files.len());

    files.into_par_iter().for_each_with(storage, |storage, path| {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let rel_path = path.to_string_lossy().to_string();
            let chunk = MirrChunk::new(
                format!("file:{}", rel_path),
                ChunkType::Module,
                content,
                rel_path.clone(),
                None,
                (1, 1),
            );

            // Generate embedding on the fly
            let vector = tokio::task::block_in_place(|| {
                futures::executor::block_on(provider.embed(&chunk.text))
            })
            .ok();

            if let Err(e) = storage.upsert_chunk(&chunk, &rel_path, vector.as_deref()) {
                eprintln!("Error indexing {}: {}", rel_path, e);
            } else {
                println!("Indexed {}", rel_path);
            }
        }
    });

    println!("Indexing complete.");
    Ok(())
}
