#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::retrieval::SearchResult;

const MAX_SNIPPET_LEN: usize = 2_000;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationSummary {
    pub dropped_empty: usize,
    pub sanitized: usize,
}

pub fn sanitize_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len().min(MAX_SNIPPET_LEN));
    for ch in text.chars() {
        if ch == '\0' {
            continue;
        }
        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }
        out.push(ch);
        if out.len() >= MAX_SNIPPET_LEN {
            break;
        }
    }
    out
}

pub fn validate_results(results: Vec<SearchResult>) -> (Vec<SearchResult>, ValidationSummary) {
    let mut summary = ValidationSummary::default();
    let mut validated = Vec::with_capacity(results.len());

    for mut result in results {
        if result.key.trim().is_empty() || result.snippet.trim().is_empty() {
            summary.dropped_empty += 1;
            continue;
        }

        let sanitized = sanitize_text(&result.snippet);
        if sanitized != result.snippet {
            summary.sanitized += 1;
            result.snippet = sanitized;
        }

        result.score = result.score.clamp(0.0, 1.0);
        validated.push(result);
    }

    (validated, summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_removes_null_and_controls() {
        let input = "a\0b\u{0007}c\n";
        let sanitized = sanitize_text(input);
        assert_eq!(sanitized, "abc\n");
    }

    #[test]
    fn validate_drops_empty_and_clamps_score() {
        let (results, summary) = validate_results(vec![
            SearchResult {
                key: String::new(),
                title: "x".to_string(),
                snippet: "y".to_string(),
                score: 2.0,
                source: "src".to_string(),
            },
            SearchResult {
                key: "k".to_string(),
                title: "t".to_string(),
                snippet: "s".to_string(),
                score: 2.0,
                source: "src".to_string(),
            },
        ]);

        assert_eq!(summary.dropped_empty, 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].score, 1.0);
    }
}
