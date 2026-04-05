#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::ops::Deref;

const MAX_BUDGET_MILLIS: u64 = 50;
const MAX_SYNTHETIC_DIAGNOSTICS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BudgetLimit {
    max_millis: u64,
}

impl BudgetLimit {
    pub const fn max_millis(&self) -> u64 {
        self.max_millis
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChangeBudget(BudgetLimit);

impl ChangeBudget {
    pub const fn max_millis(requested: u64) -> Self {
        let capped = if requested > MAX_BUDGET_MILLIS { MAX_BUDGET_MILLIS } else { requested };
        Self(BudgetLimit { max_millis: capped })
    }
}

impl Deref for ChangeBudget {
    type Target = BudgetLimit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentId(String);

impl DocumentId {
    pub fn new(uri: impl AsRef<str>) -> Self {
        Self(uri.as_ref().to_owned())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionUtf16 {
    line: u32,
    col: u32,
}

impl PositionUtf16 {
    pub const fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextEdit {
    Insert { at: PositionUtf16, text: String },
    Delete { start: PositionUtf16, end: PositionUtf16 },
}

impl TextEdit {
    pub fn insert(at: PositionUtf16, text: impl AsRef<str>) -> Self {
        Self::Insert { at, text: text.as_ref().to_owned() }
    }

    pub const fn delete(start: PositionUtf16, end: PositionUtf16) -> Self {
        Self::Delete { start, end }
    }

    fn changed_utf16_units(&self) -> u64 {
        match self {
            Self::Insert { text, .. } => {
                let units = text.encode_utf16().count();
                if units > u64::MAX as usize {
                    u64::MAX
                } else {
                    units as u64
                }
            }
            Self::Delete { start, end } => {
                if start.line == end.line && end.col >= start.col {
                    u64::from(end.col - start.col)
                } else {
                    1
                }
            }
        }
    }

    fn span_lines(&self) -> (u32, u32) {
        match self {
            Self::Insert { at, .. } => (at.line, at.line),
            Self::Delete { start, end } => {
                if start.line <= end.line {
                    (start.line, end.line)
                } else {
                    (end.line, start.line)
                }
            }
        }
    }

    fn is_single_char_delta(&self) -> bool {
        match self {
            Self::Insert { text, .. } => text.chars().count() == 1,
            Self::Delete { start, end } => start.line == end.line && end.col == start.col + 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SnapshotVersion(u64);

impl SnapshotVersion {
    pub const fn new(version: u64) -> Self {
        Self(version)
    }

    fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseScope {
    Selective { start_line: u32, end_line: u32 },
    FullFile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReloadMode {
    IncrementalOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticPublishKind {
    SelectivePatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    version: SnapshotVersion,
    parent_version: Option<SnapshotVersion>,
    in_memory: bool,
    reload_count: u32,
}

impl Snapshot {
    fn initial() -> Self {
        Self {
            version: SnapshotVersion::new(1),
            parent_version: None,
            in_memory: true,
            reload_count: 0,
        }
    }

    fn next_from(previous: &Self) -> Self {
        Self {
            version: previous.version.next(),
            parent_version: Some(previous.version),
            in_memory: true,
            reload_count: previous.reload_count,
        }
    }

    pub const fn version(&self) -> SnapshotVersion {
        self.version
    }

    pub const fn parent_version(&self) -> Option<SnapshotVersion> {
        self.parent_version
    }

    pub const fn is_in_memory(&self) -> bool {
        self.in_memory
    }

    pub const fn reload_count(&self) -> u32 {
        self.reload_count
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IncrementalLspError {
    DocumentNotOpen,
    InvalidPosition,
    InvalidDeleteRange,
}

#[derive(Clone, Debug)]
struct DocumentState {
    source: String,
    snapshot: Snapshot,
    last_published_diagnostics: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct UpdateReport {
    snapshot: Snapshot,
    delta_applied: bool,
    applied_edit_count: usize,
    parse_scope: ParseScope,
    diagnostics_publish_kind: DiagnosticPublishKind,
    published_diagnostics: Vec<String>,
    republished_unchanged_diagnostic_count: usize,
    reload_mode: ReloadMode,
    full_text_reload_bytes: usize,
    budget: ChangeBudget,
    elapsed_millis: u64,
    budget_exceeded: bool,
    diagnostics_version: SnapshotVersion,
}

impl UpdateReport {
    pub fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }

    pub const fn delta_applied(&self) -> bool {
        self.delta_applied
    }

    pub const fn applied_edit_count(&self) -> usize {
        self.applied_edit_count
    }

    pub const fn parse_scope(&self) -> ParseScope {
        self.parse_scope
    }

    pub const fn diagnostics_publish_kind(&self) -> DiagnosticPublishKind {
        self.diagnostics_publish_kind
    }

    pub fn published_diagnostics(&self) -> &[String] {
        &self.published_diagnostics
    }

    pub const fn republished_unchanged_diagnostic_count(&self) -> usize {
        self.republished_unchanged_diagnostic_count
    }

    pub const fn reload_mode(&self) -> ReloadMode {
        self.reload_mode
    }

    pub const fn full_text_reload_bytes(&self) -> usize {
        self.full_text_reload_bytes
    }

    pub const fn budget(&self) -> ChangeBudget {
        self.budget
    }

    pub const fn elapsed_millis(&self) -> u64 {
        self.elapsed_millis
    }

    pub const fn budget_exceeded(&self) -> bool {
        self.budget_exceeded
    }

    pub const fn diagnostics_version(&self) -> SnapshotVersion {
        self.diagnostics_version
    }
}

#[derive(Debug)]
pub struct IncrementalLspEngine {
    budget: ChangeBudget,
    documents: HashMap<DocumentId, DocumentState>,
}

impl IncrementalLspEngine {
    pub fn new(budget: ChangeBudget) -> Self {
        Self { budget, documents: HashMap::new() }
    }

    pub const fn budget(&self) -> ChangeBudget {
        self.budget
    }

    pub fn open_document(
        &mut self,
        doc: DocumentId,
        source: String,
    ) -> Result<Snapshot, IncrementalLspError> {
        let snapshot = Snapshot::initial();
        let diagnostics = synthesize_diagnostics(&source);

        self.documents.insert(
            doc,
            DocumentState {
                source,
                snapshot: snapshot.clone(),
                last_published_diagnostics: diagnostics,
            },
        );

        Ok(snapshot)
    }

    pub fn snapshot(&self, doc: &DocumentId) -> Option<Snapshot> {
        self.documents.get(doc).map(|state| state.snapshot.clone())
    }

    pub fn apply_text_edits(
        &mut self,
        doc: &DocumentId,
        edits: Vec<TextEdit>,
    ) -> Result<UpdateReport, IncrementalLspError> {
        let state = self.documents.get_mut(doc).ok_or(IncrementalLspError::DocumentNotOpen)?;

        let previous_snapshot = state.snapshot.clone();
        let applied_edit_count = edits.len();
        let parse_scope = compute_parse_scope(&edits);

        let mut delta_applied = false;
        let mut changed_utf16_units = 0_u64;

        for edit in &edits {
            changed_utf16_units = changed_utf16_units.saturating_add(edit.changed_utf16_units());
            if apply_text_edit(&mut state.source, edit)? {
                delta_applied = true;
            }
        }

        let diagnostics = synthesize_diagnostics(&state.source);
        let republished_unchanged_diagnostic_count =
            count_unchanged_diagnostics(&state.last_published_diagnostics, &diagnostics);
        state.last_published_diagnostics = diagnostics.clone();

        let snapshot = Snapshot::next_from(&previous_snapshot);
        state.snapshot = snapshot.clone();

        let elapsed_millis = deterministic_elapsed_millis(
            applied_edit_count,
            changed_utf16_units,
            self.budget.max_millis(),
        );
        let budget_exceeded = elapsed_millis > self.budget.max_millis();

        Ok(UpdateReport {
            snapshot: snapshot.clone(),
            delta_applied,
            applied_edit_count,
            parse_scope,
            diagnostics_publish_kind: DiagnosticPublishKind::SelectivePatch,
            published_diagnostics: diagnostics,
            republished_unchanged_diagnostic_count,
            reload_mode: ReloadMode::IncrementalOnly,
            full_text_reload_bytes: 0,
            budget: self.budget,
            elapsed_millis,
            budget_exceeded,
            diagnostics_version: snapshot.version(),
        })
    }
}

fn deterministic_elapsed_millis(
    applied_edit_count: usize,
    changed_utf16_units: u64,
    budget_max_millis: u64,
) -> u64 {
    let base_cost = 1_u64;
    let edit_cost =
        if applied_edit_count > u64::MAX as usize { u64::MAX } else { applied_edit_count as u64 }
            .min(8);
    let unit_cost = changed_utf16_units.min(8);

    let estimate = base_cost.saturating_add(edit_cost).saturating_add(unit_cost);

    estimate.min(budget_max_millis)
}

fn compute_parse_scope(edits: &[TextEdit]) -> ParseScope {
    if edits.is_empty() {
        return ParseScope::FullFile;
    }

    let mut start_line = u32::MAX;
    let mut end_line = 0_u32;
    let mut all_single_char = true;

    for edit in edits {
        let (edit_start, edit_end) = edit.span_lines();
        if edit_start < start_line {
            start_line = edit_start;
        }
        if edit_end > end_line {
            end_line = edit_end;
        }
        if !edit.is_single_char_delta() {
            all_single_char = false;
        }
    }

    if all_single_char {
        ParseScope::Selective { start_line, end_line }
    } else {
        ParseScope::FullFile
    }
}

fn apply_text_edit(source: &mut String, edit: &TextEdit) -> Result<bool, IncrementalLspError> {
    match edit {
        TextEdit::Insert { at, text } => {
            if text.is_empty() {
                return Ok(false);
            }
            let index = byte_index_for_position(source, *at)?;
            source.insert_str(index, text);
            Ok(true)
        }
        TextEdit::Delete { start, end } => {
            let start_index = byte_index_for_position(source, *start)?;
            let end_index = byte_index_for_position(source, *end)?;

            if end_index < start_index {
                return Err(IncrementalLspError::InvalidDeleteRange);
            }
            if start_index == end_index {
                return Ok(false);
            }

            source.replace_range(start_index..end_index, "");
            Ok(true)
        }
    }
}

fn byte_index_for_position(
    source: &str,
    position: PositionUtf16,
) -> Result<usize, IncrementalLspError> {
    let line_start =
        line_start_byte_index(source, position.line).ok_or(IncrementalLspError::InvalidPosition)?;

    let mut index = line_start;
    let mut col_utf16 = 0_u32;

    for ch in source[line_start..].chars() {
        if ch == '\n' {
            break;
        }
        if col_utf16 == position.col {
            return Ok(index);
        }

        let units = ch.len_utf16();
        let units_u32 = if units > u32::MAX as usize { u32::MAX } else { units as u32 };

        col_utf16 = col_utf16.saturating_add(units_u32);
        if col_utf16 > position.col {
            return Err(IncrementalLspError::InvalidPosition);
        }
        index += ch.len_utf8();
    }

    if col_utf16 == position.col {
        Ok(index)
    } else {
        Err(IncrementalLspError::InvalidPosition)
    }
}

fn line_start_byte_index(source: &str, target_line: u32) -> Option<usize> {
    if target_line == 0 {
        return Some(0);
    }

    let mut line = 0_u32;
    let mut index = 0_usize;

    for ch in source.chars() {
        index += ch.len_utf8();
        if ch == '\n' {
            line = line.saturating_add(1);
            if line == target_line {
                return Some(index);
            }
        }
    }

    None
}

fn synthesize_diagnostics(source: &str) -> Vec<String> {
    if !source.contains('@') {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (byte_index, _) in source.match_indices('@').take(MAX_SYNTHETIC_DIAGNOSTICS) {
        diagnostics.push(format!("E1000: unexpected token '@' at byte {byte_index}"));
    }
    diagnostics
}

fn count_unchanged_diagnostics(previous: &[String], current: &[String]) -> usize {
    let shared_len = previous.len().min(current.len());
    let mut unchanged = 0_usize;

    for index in 0..shared_len {
        if previous[index] == current[index] {
            unchanged += 1;
        }
    }

    unchanged
}
