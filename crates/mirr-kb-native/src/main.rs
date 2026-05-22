#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use mirr_kb_native::adapters::embedding::StubEmbeddingProvider;
use mirr_kb_native::expansion::ExpansionMode;
use mirr_kb_native::query_handler::{
    run_query_pipeline, run_status_pipeline, QueryPipelineRequest,
};
use mirr_kb_native::retrieval::SearchMode;
use mirr_kb_native::storage::SqliteHybridStorage;
use serde::Serialize;
use std::process::ExitCode;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "mirr-kb-native", version, about = "MIRR Knowledge Base Grounding Engine")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query the knowledge base with natural language
    Query {
        /// Query text
        #[arg(short, long)]
        text: String,

        /// Search mode
        #[arg(short, long, value_enum, default_value_t = SearchMode::Hybrid)]
        mode: SearchMode,

        /// Maximum results to return
        #[arg(short, long, default_value_t = 16)]
        limit: usize,

        /// Filter expression (e.g. "module:foo")
        #[arg(short, long)]
        filter: Option<String>,

        /// Expansion mode for query variants
        #[arg(long, value_enum, default_value_t = ExpansionMode::None)]
        expand_mode: ExpansionMode,

        /// Number of retries on transient errors
        #[arg(long, default_value_t = 1)]
        retry_count: u8,

        /// Query timeout in milliseconds
        #[arg(long, default_value_t = 30000)]
        timeout_ms: u64,

        /// Temporal range start (unix seconds)
        #[arg(long)]
        start_secs: Option<u64>,

        /// Temporal range end (unix seconds)
        #[arg(long)]
        end_secs: Option<u64>,
    },
    /// Generate a brief, AI-friendly summary of grounded evidence
    Brief {
        /// Query text
        #[arg(short, long)]
        query: String,

        /// Search mode
        #[arg(short, long, value_enum, default_value_t = SearchMode::Hybrid)]
        mode: SearchMode,

        /// Maximum results to return
        #[arg(short, long, default_value_t = 8)]
        limit: usize,

        /// Filter scope
        #[arg(short, long)]
        scope: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = BriefFormat::Brief)]
        format: BriefFormat,
    },
    /// Show current index status and metadata
    Status,
}

#[derive(ValueEnum, Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BriefFormat {
    Brief,
    Bullet,
    Decision,
}

impl BriefFormat {
    fn as_str(self) -> &'static str {
        match self {
            Self::Brief => "brief",
            Self::Bullet => "bullet",
            Self::Decision => "decision",
        }
    }
}

#[derive(Debug, Serialize)]
struct BriefEvidence {
    key: String,
    title: String,
    snippet: String,
    score: f32,
    source: String,
}

#[derive(Debug, Serialize)]
struct BriefResponse {
    query: String,
    mode: String,
    scope: Option<String>,
    format: String,
    answer: String,
    evidence: Vec<BriefEvidence>,
    confidence: String,
    gaps: Vec<String>,
    follow_ups: Vec<String>,
    freshness: mirr_kb_native::retrieval::Freshness,
    query_time_ms: u64,
    truncated: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let args = Cli::parse();

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
        let cmd = Cli::command();
        println!("{}", serde_json::to_string_pretty(&get_cmd_manifest(&cmd)).unwrap());
        return ExitCode::SUCCESS;
    }

    let command = match args.command {
        Some(cmd) => cmd,
        None => {
            eprintln!("Error: no command specified.\nRun with --help for usage.");
            return ExitCode::from(1);
        }
    };

    let kb_root = std::env::var("MIRR_KB_ROOT").unwrap_or_else(|_| ".kb".to_string());
    let storage = match SqliteHybridStorage::new(&kb_root) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Error opening KB storage: {}", e);
            return ExitCode::from(3);
        }
    };

    match command {
        Commands::Query {
            text,
            mode,
            limit,
            filter,
            expand_mode,
            retry_count,
            timeout_ms,
            start_secs,
            end_secs,
        } => {
            let request = QueryPipelineRequest {
                text,
                mode,
                limit,
                filter,
                expansion_mode: expand_mode,
                retry_count,
                timeout_ms,
                temporal_start_secs: start_secs,
                temporal_end_secs: end_secs,
            };
            match run_query_pipeline(storage, &StubEmbeddingProvider, request).await {
                Ok(response) => {
                    println!("{}", serde_json::to_string(&response).unwrap());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Query failed: {}", e);
                    ExitCode::from(4)
                }
            }
        }
        Commands::Brief { query, mode, limit, scope, format } => {
            let request = QueryPipelineRequest {
                text: query.clone(),
                mode,
                limit,
                filter: scope.clone(),
                expansion_mode: ExpansionMode::None,
                retry_count: 1,
                timeout_ms: 30_000,
                temporal_start_secs: None,
                temporal_end_secs: None,
            };
            match run_query_pipeline(storage, &StubEmbeddingProvider, request).await {
                Ok(response) => {
                    let output = build_brief_response(&query, mode, scope, format, response);
                    println!("{}", serde_json::to_string(&output).unwrap());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Brief failed: {}", e);
                    ExitCode::from(5)
                }
            }
        }
        Commands::Status => match run_status_pipeline(storage.as_ref()) {
            Ok(status) => {
                println!("{}", serde_json::to_string(&status).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("Status failed: {}", e);
                ExitCode::from(6)
            }
        },
    }
}

fn build_brief_response(
    query: &str,
    mode: SearchMode,
    scope: Option<String>,
    format: BriefFormat,
    response: mirr_kb_native::retrieval::QueryResponse,
) -> BriefResponse {
    let evidence = response
        .results
        .iter()
        .take(5)
        .map(|result| BriefEvidence {
            key: result.key.clone(),
            title: result.title.clone(),
            snippet: result.snippet.clone(),
            score: result.score,
            source: result.source.clone(),
        })
        .collect::<Vec<_>>();

    let confidence = brief_confidence(&evidence, response.truncated, response.freshness);
    let gaps = brief_gaps(&response, scope.as_deref());
    let follow_ups = brief_follow_ups(&evidence, scope.as_deref());
    let answer = brief_answer(query, &evidence, confidence.as_str(), format);

    BriefResponse {
        query: query.to_string(),
        mode: mode_to_string(mode).to_string(),
        scope,
        format: format.as_str().to_string(),
        answer,
        evidence,
        confidence,
        gaps,
        follow_ups,
        freshness: response.freshness,
        query_time_ms: response.query_time_ms,
        truncated: response.truncated,
    }
}

fn brief_confidence(
    evidence: &[BriefEvidence],
    truncated: bool,
    freshness: mirr_kb_native::retrieval::Freshness,
) -> String {
    if evidence.is_empty() {
        return "low".to_string();
    }

    let top_score = evidence.first().map(|item| item.score).unwrap_or(0.0);
    let mut score = if top_score >= 0.8 {
        "high"
    } else if top_score >= 0.4 {
        "medium"
    } else {
        "low"
    };

    if truncated || matches!(freshness, mirr_kb_native::retrieval::Freshness::Stale) {
        score = if score == "high" { "medium" } else { "low" };
    }

    score.to_string()
}

fn brief_gaps(
    response: &mirr_kb_native::retrieval::QueryResponse,
    scope: Option<&str>,
) -> Vec<String> {
    let mut gaps = Vec::new();

    if response.results.is_empty() {
        gaps.push("No grounded evidence retrieved for this query.".to_string());
    }
    if response.truncated {
        gaps.push("Evidence list was truncated by the configured limit.".to_string());
    }
    if matches!(response.freshness, mirr_kb_native::retrieval::Freshness::Stale) {
        gaps.push("KB index may be stale.".to_string());
    }
    if let Some(error) = response.error.as_ref() {
        gaps.push(error.clone());
    }
    if let Some(scope) = scope {
        if scope.is_empty() {
            gaps.push("Scope was empty after parsing.".to_string());
        }
    }

    gaps
}

fn brief_follow_ups(evidence: &[BriefEvidence], scope: Option<&str>) -> Vec<String> {
    let mut follow_ups = Vec::new();

    if let Some(top) = evidence.first() {
        follow_ups.push(format!("Inspect {} for the highest-confidence evidence.", top.source));
        follow_ups.push(format!("Search related symbols around {}.", top.title));
    }
    if let Some(scope) = scope {
        if !scope.is_empty() {
            follow_ups.push(format!("Narrow the next query to scope '{}'.", scope));
        }
    }

    if follow_ups.is_empty() {
        follow_ups.push("Run a narrower query or index the repository again.".to_string());
    }

    follow_ups
}

fn brief_answer(
    query: &str,
    evidence: &[BriefEvidence],
    confidence: &str,
    format: BriefFormat,
) -> String {
    if evidence.is_empty() {
        return format!("No grounded evidence was found for '{}'.", query);
    }

    let top_titles =
        evidence.iter().take(3).map(|item| item.title.as_str()).collect::<Vec<_>>().join(", ");
    let top_source = &evidence[0].source;

    match format {
        BriefFormat::Brief => format!(
            "Grounded evidence for '{}' points to {} (confidence: {}). Top source: {}.",
            query, top_titles, confidence, top_source
        ),
        BriefFormat::Bullet => format!(
            "- query: {}\n- evidence: {}\n- confidence: {}\n- top source: {}",
            query, top_titles, confidence, top_source
        ),
        BriefFormat::Decision => format!(
            "Decision: evidence suggests {}. Confidence: {}. Top source: {}.",
            top_titles, confidence, top_source
        ),
    }
}

fn mode_to_string(mode: SearchMode) -> &'static str {
    match mode {
        SearchMode::Lexical => "lexical",
        SearchMode::Semantic => "semantic",
        SearchMode::Hybrid => "hybrid",
        SearchMode::Graph => "graph",
        SearchMode::Temporal => "temporal",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mirr_kb_native::retrieval::{Freshness, QueryResponse, SearchResult};

    #[test]
    fn brief_response_includes_grounded_summary_fields() {
        let response = QueryResponse {
            results: vec![SearchResult {
                key: "demo.alpha".to_string(),
                title: "demo.alpha".to_string(),
                snippet: "alpha evidence".to_string(),
                score: 0.91,
                source: "src/demo.mirr".to_string(),
            }],
            freshness: Freshness::Fresh,
            query_time_ms: 7,
            truncated: false,
            error: None,
        };

        let brief = build_brief_response(
            "alpha",
            SearchMode::Hybrid,
            Some("src".to_string()),
            BriefFormat::Decision,
            response,
        );

        assert_eq!(brief.evidence.len(), 1);
        assert_eq!(brief.confidence, "high");
        assert!(brief.answer.contains("Decision"));
        assert!(brief.follow_ups.iter().any(|item| item.contains("src")));
    }
}
