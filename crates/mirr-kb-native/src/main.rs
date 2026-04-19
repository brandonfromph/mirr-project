#![forbid(unsafe_code)]
#![deny(warnings)]

use mirr_kb_native::adapters::embedding::StubEmbeddingProvider;
use mirr_kb_native::expansion::ExpansionMode;
use mirr_kb_native::query_handler::{
    run_query_pipeline, run_status_pipeline, QueryPipelineRequest,
};
use mirr_kb_native::retrieval::SearchMode;
use mirr_kb_native::storage::SqliteHybridStorage;
use std::process::ExitCode;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match run_cli().await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(2)
        }
    }
}

async fn run_cli() -> anyhow::Result<ExitCode> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow::anyhow!("usage: mirr-kb-native <query|status> [options]"));
    }

    let kb_root = std::env::var("MIRR_KB_ROOT").unwrap_or_else(|_| ".kb".to_string());
    let storage = Arc::new(SqliteHybridStorage::new(&kb_root)?);

    match args[0].as_str() {
        "query" => run_query_command(storage, &args[1..]).await,
        "status" => {
            let status = run_status_pipeline(storage.as_ref())?;
            println!("{}", serde_json::to_string(&status)?);
            Ok(ExitCode::SUCCESS)
        }
        _ => Err(anyhow::anyhow!("unknown command: {}", args[0])),
    }
}

async fn run_query_command(
    storage: Arc<SqliteHybridStorage>,
    args: &[String],
) -> anyhow::Result<ExitCode> {
    let mut text = String::new();
    let mut mode = SearchMode::Hybrid;
    let mut limit: usize = 16;
    let mut filter: Option<String> = None;
    let mut expansion_mode = ExpansionMode::None;
    let mut retry_count: u8 = 1;
    let mut timeout_ms: u64 = 30_000;
    let mut temporal_start_secs: Option<u64> = None;
    let mut temporal_end_secs: Option<u64> = None;

    let mut i = 0usize;
    while i < args.len() {
        let flag = args[i].as_str();
        let next = if i + 1 < args.len() { Some(args[i + 1].as_str()) } else { None };

        match (flag, next) {
            ("--text", Some(value)) => {
                text = value.to_string();
                i += 2;
            }
            ("--mode", Some(value)) => {
                mode = parse_mode(value)?;
                i += 2;
            }
            ("--limit", Some(value)) => {
                limit = value.parse::<usize>().map_err(|_| anyhow::anyhow!("invalid --limit"))?;
                i += 2;
            }
            ("--filter", Some(value)) => {
                filter = Some(value.to_string());
                i += 2;
            }
            ("--expand-mode", Some(value)) => {
                expansion_mode = ExpansionMode::parse(value)
                    .ok_or_else(|| anyhow::anyhow!("invalid --expand-mode"))?;
                i += 2;
            }
            ("--retry-count", Some(value)) => {
                retry_count =
                    value.parse::<u8>().map_err(|_| anyhow::anyhow!("invalid --retry-count"))?;
                i += 2;
            }
            ("--timeout-ms", Some(value)) => {
                timeout_ms =
                    value.parse::<u64>().map_err(|_| anyhow::anyhow!("invalid --timeout-ms"))?;
                i += 2;
            }
            ("--start-secs", Some(value)) => {
                temporal_start_secs = Some(
                    value.parse::<u64>().map_err(|_| anyhow::anyhow!("invalid --start-secs"))?,
                );
                i += 2;
            }
            ("--end-secs", Some(value)) => {
                temporal_end_secs =
                    Some(value.parse::<u64>().map_err(|_| anyhow::anyhow!("invalid --end-secs"))?);
                i += 2;
            }
            _ => {
                return Err(anyhow::anyhow!("unknown or incomplete flag: {}", flag));
            }
        }
    }

    let request = QueryPipelineRequest {
        text,
        mode,
        limit,
        filter,
        expansion_mode,
        retry_count,
        timeout_ms,
        temporal_start_secs,
        temporal_end_secs,
    };

    let response = run_query_pipeline(storage, &StubEmbeddingProvider, request).await?;
    println!("{}", serde_json::to_string(&response)?);
    // Always return success if the query ran without error, even if results are empty.
    Ok(ExitCode::SUCCESS)
}

fn parse_mode(value: &str) -> anyhow::Result<SearchMode> {
    match value.to_ascii_lowercase().as_str() {
        "lexical" => Ok(SearchMode::Lexical),
        "semantic" => Ok(SearchMode::Semantic),
        "hybrid" => Ok(SearchMode::Hybrid),
        "graph" => Ok(SearchMode::Graph),
        "temporal" => Ok(SearchMode::Temporal),
        _ => Err(anyhow::anyhow!("invalid --mode")),
    }
}
