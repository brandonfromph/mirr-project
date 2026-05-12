#![forbid(unsafe_code)]
#![deny(warnings)]

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
        return Err(anyhow::anyhow!("usage: mirr-kb-native <query|status|brief> [options]"));
    }

    let kb_root = std::env::var("MIRR_KB_ROOT").unwrap_or_else(|_| ".kb".to_string());
    let storage = Arc::new(SqliteHybridStorage::new(&kb_root)?);

    match args[0].as_str() {
        "query" => run_query_command(storage, &args[1..]).await,
        "brief" => run_brief_command(storage, &args[1..]).await,
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

async fn run_brief_command(
    storage: Arc<SqliteHybridStorage>,
    args: &[String],
) -> anyhow::Result<ExitCode> {
    let mut query = String::new();
    let mut mode = SearchMode::Hybrid;
    let mut limit: usize = 8;
    let mut scope: Option<String> = None;
    let mut format = BriefFormat::Brief;

    let mut i = 0usize;
    while i < args.len() {
        let flag = args[i].as_str();
        let next = if i + 1 < args.len() { Some(args[i + 1].as_str()) } else { None };

        match (flag, next) {
            ("--query", Some(value)) => {
                query = value.to_string();
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
            ("--scope", Some(value)) => {
                scope = Some(value.to_string());
                i += 2;
            }
            ("--format", Some(value)) => {
                format = BriefFormat::parse(value)?;
                i += 2;
            }
            _ => {
                return Err(anyhow::anyhow!("unknown or incomplete flag: {}", flag));
            }
        }
    }

    if query.is_empty() {
        return Err(anyhow::anyhow!("missing --query"));
    }

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

    let response = run_query_pipeline(storage, &StubEmbeddingProvider, request).await?;
    let output = build_brief_response(&query, mode, scope, format, response);
    println!("{}", serde_json::to_string(&output)?);
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BriefFormat {
    Brief,
    Bullet,
    Decision,
}

impl BriefFormat {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "brief" => Ok(Self::Brief),
            "bullet" => Ok(Self::Bullet),
            "decision" => Ok(Self::Decision),
            _ => Err(anyhow::anyhow!("invalid --format")),
        }
    }

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
