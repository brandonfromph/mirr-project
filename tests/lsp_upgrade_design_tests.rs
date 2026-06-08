#![forbid(unsafe_code)]

use mirrc::lsp_incremental::{
    ChangeBudget, DiagnosticPublishKind, DocumentId, IncrementalLspEngine, ParseScope,
    PositionUtf16, ReloadMode, SnapshotVersion, TextEdit,
};

fn baseline_source() -> String {
    "module Main {\n  signal q: bool;\n  reflex q <= false;\n}\n".to_owned()
}

fn seeded_engine() -> (IncrementalLspEngine, DocumentId) {
    let mut engine = IncrementalLspEngine::new(ChangeBudget::max_millis(50));
    let doc = DocumentId::new("file:///wave4/main.mirr");
    let _ = engine
        .open_document(DocumentId::new("file:///wave4/main.mirr"), baseline_source())
        .expect("open document should produce initial in-memory snapshot");
    (engine, doc)
}

fn single_char_insert() -> TextEdit {
    TextEdit::insert(PositionUtf16::new(1, 11), "x")
}

fn single_char_delete() -> TextEdit {
    TextEdit::delete(PositionUtf16::new(1, 11), PositionUtf16::new(1, 12))
}

#[test]
fn open_document_creates_in_memory_ast_snapshot() {
    let mut engine = IncrementalLspEngine::new(ChangeBudget::max_millis(50));
    let snapshot = engine
        .open_document(DocumentId::new("file:///wave4/main.mirr"), baseline_source())
        .expect("open_document must build an in-memory AST snapshot");

    assert_eq!(snapshot.version(), SnapshotVersion::new(1));
    assert!(snapshot.is_in_memory());
}

#[test]
fn snapshot_lookup_returns_open_document_without_reload() {
    let (engine, doc) = seeded_engine();
    let snapshot = engine.snapshot(&doc).expect("opened document must be discoverable in memory");

    assert_eq!(snapshot.version(), SnapshotVersion::new(1));
    assert_eq!(snapshot.reload_count(), 0);
}

#[test]
fn initial_snapshot_has_no_parent_version() {
    let (engine, doc) = seeded_engine();
    let snapshot = engine.snapshot(&doc).expect("seeded snapshot should exist in memory");

    assert_eq!(snapshot.parent_version(), None);
}

#[test]
fn single_char_insert_increments_snapshot_version() {
    let (mut engine, doc) = seeded_engine();
    let before = engine.snapshot(&doc).expect("pre-edit snapshot must exist");

    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char insertion should apply incrementally");

    assert_ne!(report.snapshot().version(), before.version());
}

#[test]
fn single_char_insert_records_previous_version_as_parent() {
    let (mut engine, doc) = seeded_engine();
    let before = engine.snapshot(&doc).expect("pre-edit snapshot must exist");

    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char insertion should produce a new snapshot lineage");

    assert_eq!(report.snapshot().parent_version(), Some(before.version()));
}

#[test]
fn single_char_diff_reports_delta_applied() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("incremental insert should apply as a diff");

    assert!(report.delta_applied());
}

#[test]
fn single_char_delete_reports_delta_applied() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_delete()])
        .expect("incremental delete should apply as a diff");

    assert!(report.delta_applied());
}

#[test]
fn batched_edits_report_exact_applied_edit_count() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(
            &doc,
            vec![
                TextEdit::insert(PositionUtf16::new(1, 11), "x"),
                TextEdit::insert(PositionUtf16::new(1, 12), "y"),
            ],
        )
        .expect("batched edits should preserve packet count");

    assert_eq!(report.applied_edit_count(), 2);
}

#[test]
fn parse_scope_for_single_char_edit_is_selective() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char edit should trigger selective reparse");

    assert!(matches!(report.parse_scope(), ParseScope::Selective { .. }));
}

#[test]
fn parse_scope_never_reports_full_file_for_single_char_edit() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char edit should avoid full-file parse scope");

    assert!(!matches!(report.parse_scope(), ParseScope::FullFile));
}

#[test]
fn diagnostics_publish_kind_for_incremental_edit_is_selective_patch() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("incremental edit should publish targeted diagnostics");

    assert_eq!(report.diagnostics_publish_kind(), DiagnosticPublishKind::SelectivePatch);
}

#[test]
fn diagnostics_publish_batch_is_non_empty_after_error_introducing_edit() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![TextEdit::insert(PositionUtf16::new(0, 0), "@")])
        .expect("error-inducing edit should produce publishable diagnostics");

    assert!(!report.published_diagnostics().is_empty());
}

#[test]
fn unchanged_diagnostics_are_not_republished() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![TextEdit::insert(PositionUtf16::new(3, 1), " ")])
        .expect("whitespace edit should not republish unchanged diagnostics");

    assert_eq!(report.republished_unchanged_diagnostic_count(), 0);
}

#[test]
fn single_char_edit_uses_incremental_reload_mode() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char edit should stay on incremental reload path");

    assert_eq!(report.reload_mode(), ReloadMode::IncrementalOnly);
}

#[test]
fn single_char_edit_reports_zero_full_text_reload_bytes() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char edit should not request full-text payload reload");

    assert_eq!(report.full_text_reload_bytes(), 0);
}

#[test]
fn engine_budget_contract_is_capped_at_fifty_millis() {
    let engine = IncrementalLspEngine::new(ChangeBudget::max_millis(50));

    assert!(engine.budget().max_millis() <= 50);
}

#[test]
fn update_report_surfaces_budget_contract_at_or_below_fifty_millis() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("incremental report should carry budget contract");

    assert!(report.budget().max_millis() <= 50);
}

#[test]
fn update_report_surfaces_elapsed_millis_metric() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("incremental report should expose elapsed latency metric");

    assert!(report.elapsed_millis() <= report.budget().max_millis());
}

#[test]
fn non_pathological_single_char_edit_stays_within_budget() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("single-char edit should remain under the budget contract");

    assert!(!report.budget_exceeded());
}

#[test]
fn update_report_publishes_snapshot_and_diagnostics_same_version() {
    let (mut engine, doc) = seeded_engine();
    let report = engine
        .apply_text_edits(&doc, vec![single_char_insert()])
        .expect("diagnostics publish must align with snapshot version");

    assert_eq!(report.snapshot().version(), report.diagnostics_version());
}
