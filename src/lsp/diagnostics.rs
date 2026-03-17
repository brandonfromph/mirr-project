//! Convert MIRR compiler errors into LSP diagnostic JSON objects.

#![forbid(unsafe_code)]

use serde_json::{json, Value};

use crate::error::MirrError;

/// Convert a `MirrError` into an LSP-compatible diagnostics array (JSON).
pub fn mirr_error_to_diagnostics(error: &MirrError) -> Vec<Value> {
    let range = error_range(error);
    let severity = error_severity(error);
    let code = error_code(error);
    let message = error.to_string();

    let mut diag = json!({
        "range": range,
        "severity": severity,
        "source": "mirr",
        "message": message,
    });

    if let Some(c) = code {
        diag["code"] = json!(c);
    }

    vec![diag]
}

/// Build an empty diagnostics publish notification (clears errors).
pub fn clear_diagnostics(uri: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": [],
        }
    })
}

/// Build a diagnostics publish notification.
pub fn publish_diagnostics(uri: &str, diagnostics: &[Value]) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diagnostics,
        }
    })
}

/// Map a `MirrError`'s span to an LSP Range JSON object.
fn error_range(error: &MirrError) -> Value {
    match error.span() {
        Some(span) => {
            let end_char = if span.end_col == u32::MAX { 0 } else { span.end_col };
            let end_line = if span.end_col == u32::MAX { span.end_line + 1 } else { span.end_line };
            json!({
                "start": { "line": span.start_line, "character": span.start_col },
                "end": { "line": end_line, "character": end_char },
            })
        }
        None => json!({
            "start": { "line": 0, "character": 0 },
            "end": { "line": 0, "character": 0 },
        }),
    }
}

/// Map error variant to LSP DiagnosticSeverity (1=Error, 2=Warning).
fn error_severity(error: &MirrError) -> u32 {
    match error {
        MirrError::RspuError { .. } => 2, // Warning
        _ => 1,                           // Error
    }
}

/// Extract a string error code from the error.
///
/// Delegates to [`MirrError::error_code`] which inspects embedded `[Ennn]`
/// codes and falls back to per-variant defaults.
pub fn error_code(error: &MirrError) -> Option<String> {
    error.error_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn error_without_span_maps_to_zero_range() {
        let err = MirrError::parse_error("test error");
        let diags = mirr_error_to_diagnostics(&err);
        assert_eq!(diags.len(), 1);
        let range = &diags[0]["range"];
        assert_eq!(range["start"]["line"], 0);
        assert_eq!(range["start"]["character"], 0);
    }

    #[test]
    fn error_with_span_maps_to_correct_range() {
        let err = MirrError::SemanticError {
            message: "[E201] Duplicate signal.".to_string(),
            span: Some(Span::full_line(5)),
        };
        let diags = mirr_error_to_diagnostics(&err);
        assert_eq!(diags.len(), 1);
        let range = &diags[0]["range"];
        assert_eq!(range["start"]["line"], 5);
        assert_eq!(range["start"]["character"], 0);
        // full_line uses end_col=MAX, which maps to next line start
        assert_eq!(range["end"]["line"], 6);
        assert_eq!(range["end"]["character"], 0);
    }

    #[test]
    fn semantic_error_severity_is_error() {
        let err = MirrError::SemanticError { message: "test".to_string(), span: None };
        assert_eq!(error_severity(&err), 1);
    }

    #[test]
    fn rspu_error_severity_is_warning() {
        let err = MirrError::RspuError { message: "test".to_string(), span: None };
        assert_eq!(error_severity(&err), 2);
    }

    #[test]
    fn error_code_extracted_from_message() {
        let err = MirrError::SemanticError {
            message: "[E201] Duplicate signal name: 'x'.".to_string(),
            span: None,
        };
        assert_eq!(error_code(&err), Some("E201".to_string()));
    }

    #[test]
    fn error_code_none_when_absent() {
        let err = MirrError::parse_error("no code here");
        // ParseError falls back to E100 via MirrError::error_code().
        assert_eq!(error_code(&err), Some("E100".to_string()));
    }
}
