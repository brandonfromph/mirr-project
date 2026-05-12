//! Synchronous LSP server for the MIRR compiler.
//!
//! Reads JSON-RPC messages from stdin, dispatches them, and writes
//! responses/notifications to stdout. Zero async dependencies.
//!
//! Supported methods:
//! - `initialize` / `initialized` / `shutdown` / `exit`
//! - `textDocument/didOpen` / `didChange` / `didSave` / `didClose`

#![forbid(unsafe_code)]

use std::io::{self, BufRead, Write};

use serde_json::{json, Value};

use super::diagnostics::{clear_diagnostics, mirr_error_to_diagnostics, publish_diagnostics};
use super::transport::{read_message, write_message};
use crate::lsp_incremental::{
    ChangeBudget, DocumentId, IncrementalLspEngine, IncrementalLspError, PositionUtf16, TextEdit,
};
use crate::pipeline::{run_pipeline, PipelineConfig};
use std::collections::HashMap;

/// Maximum source file size the LSP will process (1 MB).
const MAX_SOURCE_BYTES: usize = 1_048_576;

struct LspSession {
    engine: IncrementalLspEngine,
    documents: HashMap<String, String>,
}

impl LspSession {
    fn new() -> Self {
        Self {
            engine: IncrementalLspEngine::new(ChangeBudget::max_millis(50)),
            documents: HashMap::new(),
        }
    }

    fn open_document(&mut self, uri: &str, source: &str) -> Result<(), IncrementalLspError> {
        let doc = DocumentId::new(uri);
        self.documents.insert(uri.to_owned(), source.to_owned());
        let _ = self.engine.open_document(doc, source.to_owned())?;
        Ok(())
    }

    fn sync_document(&mut self, uri: &str, source: &str) -> Result<(), IncrementalLspError> {
        let doc = DocumentId::new(uri);
        match self.documents.insert(uri.to_owned(), source.to_owned()) {
            None => {
                let _ = self.engine.open_document(doc, source.to_owned())?;
            }
            Some(previous_source) => {
                let edits = full_sync_replacement_edits(&previous_source, source);
                if !edits.is_empty() {
                    let _ = self.engine.apply_text_edits(&doc, edits)?;
                }
            }
        }
        Ok(())
    }

    fn close_document(&mut self, uri: &str) {
        self.documents.remove(uri);
        let _ = self.engine.close_document(&DocumentId::new(uri));
    }
}

/// Run the LSP server loop, reading from `input` and writing to `output`.
///
/// Returns when the client sends `exit` or the input stream closes.
pub fn run(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<()> {
    let mut session = LspSession::new();
    let mut shutdown_requested = false;

    loop {
        let msg = match read_message(input)? {
            Some(m) => m,
            None => return Ok(()), // EOF
        };

        let request: Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(_) => continue, // Malformed JSON — skip.
        };

        let method = request["method"].as_str().unwrap_or("");
        let id = request.get("id").cloned();

        match method {
            "initialize" => {
                if let Some(req_id) = id {
                    let result = json!({
                        "capabilities": {
                            "textDocumentSync": {
                                "openClose": true,
                                "change": 1,
                                "save": { "includeText": true }
                            }
                        },
                        "serverInfo": {
                            "name": "mirr-lsp",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    });
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": result,
                    });
                    write_message(output, &response.to_string())?;
                }
            }

            "initialized" => {
                // No action needed.
            }

            "shutdown" => {
                shutdown_requested = true;
                if let Some(req_id) = id {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null,
                    });
                    write_message(output, &response.to_string())?;
                }
            }

            "exit" => {
                return if shutdown_requested {
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "exit without shutdown"))
                };
            }

            "textDocument/didOpen" => {
                if let Some(params) = request.get("params") {
                    if let Some(doc) = params.get("textDocument") {
                        let uri = doc["uri"].as_str().unwrap_or("");
                        let text = doc["text"].as_str().unwrap_or("");
                        let _ = session.open_document(uri, text);
                        let notification = compile_and_diagnose(uri, text);
                        write_message(output, &notification.to_string())?;
                    }
                }
            }

            "textDocument/didChange" => {
                if let Some(params) = request.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    // Full sync: take the last content change.
                    if let Some(changes) = params["contentChanges"].as_array() {
                        if let Some(last) = changes.last() {
                            let text = last["text"].as_str().unwrap_or("");
                            let _ = session.sync_document(uri, text);
                            let notification = compile_and_diagnose(uri, text);
                            write_message(output, &notification.to_string())?;
                        }
                    }
                }
            }

            "textDocument/didSave" => {
                if let Some(params) = request.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if let Some(text) = params.get("text").and_then(|t| t.as_str()) {
                        let _ = session.sync_document(uri, text);
                        let notification = compile_and_diagnose(uri, text);
                        write_message(output, &notification.to_string())?;
                    }
                }
            }

            "textDocument/didClose" => {
                if let Some(params) = request.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    session.close_document(uri);
                    let notification = clear_diagnostics(uri);
                    write_message(output, &notification.to_string())?;
                }
            }

            _ => {
                // Unknown method. If it has an id, respond with MethodNotFound.
                if let Some(req_id) = id {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "error": {
                            "code": -32601,
                            "message": format!("Method not found: {method}"),
                        }
                    });
                    write_message(output, &response.to_string())?;
                }
                // Notifications without id are silently ignored.
            }
        }
    }
}

/// Compile MIRR source and return a publishDiagnostics notification.
fn compile_and_diagnose(uri: &str, source: &str) -> Value {
    if source.len() > MAX_SOURCE_BYTES {
        let diag = json!({
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": 0, "character": 0 },
            },
            "severity": 2,
            "source": "mirr",
            "message": format!("File exceeds {} byte limit for LSP analysis.", MAX_SOURCE_BYTES),
        });
        return publish_diagnostics(uri, &[diag]);
    }

    let config = PipelineConfig {
        typecheck: true,
        simplify: false,
        width: true,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    match run_pipeline(source, &config) {
        Ok(_result) => {
            // Success — clear diagnostics.
            clear_diagnostics(uri)
        }
        Err(e) => {
            let diags: Vec<serde_json::Value> =
                e.errors.iter().flat_map(mirr_error_to_diagnostics).collect();
            publish_diagnostics(uri, &diags)
        }
    }
}

fn full_sync_replacement_edits(previous: &str, current: &str) -> Vec<TextEdit> {
    if previous == current {
        return Vec::new();
    }

    let prefix_bytes = common_prefix_bytes(previous, current);
    let suffix_bytes = common_suffix_bytes(previous, current, prefix_bytes);

    let previous_change_end = previous.len().saturating_sub(suffix_bytes);
    let current_change_end = current.len().saturating_sub(suffix_bytes);

    let start = position_for_byte_index(previous, prefix_bytes);
    let end = position_for_byte_index(previous, previous_change_end);
    let replacement = &current[prefix_bytes..current_change_end];

    let mut edits = Vec::with_capacity(2);
    edits.push(TextEdit::delete(start, end));
    if !replacement.is_empty() {
        edits.push(TextEdit::insert(start, replacement));
    }
    edits
}

fn common_prefix_bytes(previous: &str, current: &str) -> usize {
    let mut prefix = 0_usize;
    for (left, right) in previous.chars().zip(current.chars()) {
        if left != right {
            break;
        }
        prefix += left.len_utf8();
    }
    prefix
}

fn common_suffix_bytes(previous: &str, current: &str, prefix_bytes: usize) -> usize {
    let previous_tail = &previous[prefix_bytes..];
    let current_tail = &current[prefix_bytes..];
    let mut suffix = 0_usize;

    for (left, right) in previous_tail.chars().rev().zip(current_tail.chars().rev()) {
        if left != right {
            break;
        }
        suffix += left.len_utf8();
    }

    suffix
}

fn position_for_byte_index(source: &str, byte_index: usize) -> PositionUtf16 {
    let mut line = 0_u32;
    let mut col = 0_u32;
    let mut index = 0_usize;

    for ch in source.chars() {
        if index >= byte_index {
            break;
        }

        if ch == '\n' {
            line = line.saturating_add(1);
            col = 0;
        } else {
            let units = ch.len_utf16();
            let units_u32 = if units > u32::MAX as usize { u32::MAX } else { units as u32 };
            col = col.saturating_add(units_u32);
        }

        index += ch.len_utf8();
    }

    PositionUtf16::new(line, col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_request(method: &str, id: Option<u32>, params: Value) -> String {
        let mut msg = json!({
            "jsonrpc": "2.0",
            "method": method,
        });
        if let Some(i) = id {
            msg["id"] = json!(i);
        }
        msg["params"] = params;
        msg.to_string()
    }

    fn frame(body: &str) -> String {
        format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
    }

    #[test]
    fn initialize_returns_capabilities() {
        let init = make_request("initialize", Some(1), json!({"capabilities": {}}));
        let shutdown = make_request("shutdown", Some(2), json!(null));
        let exit = make_request("exit", None, json!(null));

        let input_str = format!("{}{}{}", frame(&init), frame(&shutdown), frame(&exit));
        let mut input = Cursor::new(input_str.into_bytes());
        let mut output: Vec<u8> = Vec::new();

        run(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("mirr-lsp"), "should contain server name");
        assert!(output_str.contains("textDocumentSync"), "should contain sync capability");
    }

    #[test]
    fn did_open_publishes_diagnostics_for_valid_source() {
        let init = make_request("initialize", Some(1), json!({"capabilities": {}}));
        let open = make_request(
            "textDocument/didOpen",
            None,
            json!({
                "textDocument": {
                    "uri": "file:///test.mirr",
                    "languageId": "mirr",
                    "version": 1,
                    "text": "module m {\n    signal x: in bool;\n    guard g {\n        when x\n        for 1 cycles;\n    }\n}\n"
                }
            }),
        );
        let shutdown = make_request("shutdown", Some(2), json!(null));
        let exit = make_request("exit", None, json!(null));

        let input_str =
            format!("{}{}{}{}", frame(&init), frame(&open), frame(&shutdown), frame(&exit),);
        let mut input = Cursor::new(input_str.into_bytes());
        let mut output: Vec<u8> = Vec::new();

        run(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("publishDiagnostics"), "should publish diagnostics");
        assert!(
            output_str.contains("\"diagnostics\":[]"),
            "valid source should have empty diagnostics"
        );
    }

    #[test]
    fn did_open_publishes_error_for_invalid_source() {
        let init = make_request("initialize", Some(1), json!({"capabilities": {}}));
        let open = make_request(
            "textDocument/didOpen",
            None,
            json!({
                "textDocument": {
                    "uri": "file:///bad.mirr",
                    "languageId": "mirr",
                    "version": 1,
                    "text": "this is not valid mirr"
                }
            }),
        );
        let shutdown = make_request("shutdown", Some(2), json!(null));
        let exit = make_request("exit", None, json!(null));

        let input_str =
            format!("{}{}{}{}", frame(&init), frame(&open), frame(&shutdown), frame(&exit),);
        let mut input = Cursor::new(input_str.into_bytes());
        let mut output: Vec<u8> = Vec::new();

        run(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("publishDiagnostics"), "should publish diagnostics");
        assert!(
            output_str.contains("\"severity\":1"),
            "invalid source should have severity=1 (error)"
        );
    }
}
