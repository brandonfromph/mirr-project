#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShadowMode {
    DualExecute,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EngineKind {
    TypeScript,
    Wasm,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MismatchPolicy {
    LogAndReturnTs,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RollbackSwitch {
    enabled: bool,
}

impl RollbackSwitch {
    pub fn in_memory(enabled: bool) -> Self {
        Self { enabled }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OutputEnvelope {
    stdout_limit: usize,
    stderr_limit: usize,
}

impl OutputEnvelope {
    pub fn bounded(stdout_limit: usize, stderr_limit: usize) -> Self {
        Self { stdout_limit, stderr_limit }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SemanticParity {
    CanonicalJson,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ByteParity {
    Exact,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompareRules {
    deterministic: bool,
    rule_version: String,
    semantic_rule: SemanticParity,
    byte_rule: ByteParity,
    ignore_fields: BTreeSet<String>,
}

impl CompareRules {
    pub fn deterministic(rule_version: &str) -> Self {
        Self {
            deterministic: true,
            rule_version: rule_version.to_string(),
            semantic_rule: SemanticParity::CanonicalJson,
            byte_rule: ByteParity::Exact,
            ignore_fields: BTreeSet::new(),
        }
    }

    pub fn semantic(mut self, semantic_rule: SemanticParity) -> Self {
        self.semantic_rule = semantic_rule;
        self
    }

    pub fn bytes(mut self, byte_rule: ByteParity) -> Self {
        self.byte_rule = byte_rule;
        self
    }

    pub fn ignore_fields<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ignore_fields.clear();
        for field in fields {
            self.ignore_fields.insert(field.into());
        }
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolCall {
    tool_name: String,
    correlation_id: Option<String>,
    arguments: BTreeMap<String, String>,
    shadow_hint: Option<bool>,
}

impl ToolCall {
    pub fn new(tool_name: &str) -> Self {
        Self {
            tool_name: tool_name.to_string(),
            correlation_id: None,
            arguments: BTreeMap::new(),
            shadow_hint: None,
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_string());
        self
    }

    pub fn with_argument(mut self, key: &str, value: &str) -> Self {
        self.arguments.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_shadow_hint(mut self, enabled: bool) -> Self {
        self.shadow_hint = Some(enabled);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticPayload {
    value: Value,
}

impl SemanticPayload {
    pub fn from_json_bytes(json_bytes: Vec<u8>) -> Result<Self, SemanticPayloadError> {
        let value = serde_json::from_slice::<Value>(&json_bytes)
            .map_err(|_| SemanticPayloadError::new("semantic_payload_invalid_json"))?;
        Ok(Self { value })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemanticPayloadError {
    code: &'static str,
}

impl SemanticPayloadError {
    fn new(code: &'static str) -> Self {
        Self { code }
    }
}

impl std::fmt::Display for SemanticPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl std::error::Error for SemanticPayloadError {}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineResult {
    success: bool,
    semantic: Option<SemanticPayload>,
    bytes: Vec<u8>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl EngineResult {
    pub fn success() -> Self {
        Self {
            success: true,
            semantic: None,
            bytes: Vec::new(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    pub fn with_semantic(mut self, semantic: SemanticPayload) -> Self {
        self.semantic = Some(semantic);
        self
    }

    pub fn with_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.bytes = bytes;
        self
    }

    pub fn with_stdout(mut self, stdout: Vec<u8>) -> Self {
        self.stdout = stdout;
        self
    }

    pub fn with_stderr(mut self, stderr: Vec<u8>) -> Self {
        self.stderr = stderr;
        self
    }
}

#[derive(Debug, Clone)]
pub struct MockEngine {
    kind: EngineKind,
    result: Option<EngineResult>,
}

impl MockEngine {
    pub fn typescript() -> Self {
        Self { kind: EngineKind::TypeScript, result: None }
    }

    pub fn wasm() -> Self {
        Self { kind: EngineKind::Wasm, result: None }
    }

    pub fn returns(mut self, result: EngineResult) -> Self {
        self.result = Some(result);
        self
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExecutionPath {
    ShadowDual,
    TsOnlyRollback,
    TsOnlyRequestHint,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RollbackReason {
    RuntimeFlag,
    RequestShadowHint,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParityClassification {
    FullMatch,
    SemanticMismatch,
    ByteMismatch,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParityStatus {
    Match,
    Mismatch,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParityDecision {
    ReturnTypeScript,
    ReturnTypeScriptWithLoggedMismatch,
    ReturnTypeScriptRollback,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ByteMismatch {
    first_offset: Option<usize>,
}

impl ByteMismatch {
    pub fn first_offset(&self) -> Option<usize> {
        self.first_offset
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParityReport {
    semantic_match: bool,
    byte_match: bool,
    classification: ParityClassification,
    decision: ParityDecision,
    rule_version: String,
    deterministic: bool,
    byte_mismatch: ByteMismatch,
}

impl ParityReport {
    pub fn semantic_match(&self) -> bool {
        self.semantic_match
    }

    pub fn byte_match(&self) -> bool {
        self.byte_match
    }

    pub fn classification(&self) -> ParityClassification {
        self.classification
    }

    pub fn decision(&self) -> ParityDecision {
        self.decision
    }

    pub fn rule_version(&self) -> &str {
        &self.rule_version
    }

    pub fn deterministic(&self) -> bool {
        self.deterministic
    }

    pub fn byte_mismatch(&self) -> ByteMismatch {
        self.byte_mismatch
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MismatchEvent {
    semantic_status: ParityStatus,
    byte_status: ParityStatus,
}

impl MismatchEvent {
    pub fn semantic_status(&self) -> ParityStatus {
        self.semantic_status
    }

    pub fn byte_status(&self) -> ParityStatus {
        self.byte_status
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvocationTrace {
    typescript_invocations: usize,
    wasm_invocations: usize,
    typescript_correlation_id: String,
    wasm_correlation_id: String,
}

impl InvocationTrace {
    pub fn typescript_invocations(&self) -> usize {
        self.typescript_invocations
    }

    pub fn wasm_invocations(&self) -> usize {
        self.wasm_invocations
    }

    pub fn typescript_correlation_id(&self) -> &str {
        &self.typescript_correlation_id
    }

    pub fn wasm_correlation_id(&self) -> &str {
        &self.wasm_correlation_id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineResponse {
    engine: EngineKind,
    body_bytes: Vec<u8>,
    semantic: Option<SemanticPayload>,
    stdout_bytes: Vec<u8>,
    stderr_bytes: Vec<u8>,
    stdout_truncated: bool,
    stderr_truncated: bool,
    output_limit_stdout_bytes: usize,
    output_limit_stderr_bytes: usize,
}

impl EngineResponse {
    pub fn engine(&self) -> EngineKind {
        self.engine
    }

    pub fn body_bytes(&self) -> Vec<u8> {
        self.body_bytes.clone()
    }

    pub fn stdout_bytes(&self) -> Vec<u8> {
        self.stdout_bytes.clone()
    }

    pub fn stderr_bytes(&self) -> Vec<u8> {
        self.stderr_bytes.clone()
    }

    pub fn stdout_truncated(&self) -> bool {
        self.stdout_truncated
    }

    pub fn stderr_truncated(&self) -> bool {
        self.stderr_truncated
    }

    pub fn output_limit_stdout_bytes(&self) -> usize {
        self.output_limit_stdout_bytes
    }

    pub fn output_limit_stderr_bytes(&self) -> usize {
        self.output_limit_stderr_bytes
    }

    fn semantic(&self) -> Option<&SemanticPayload> {
        self.semantic.as_ref()
    }

    fn body(&self) -> &[u8] {
        &self.body_bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvocationOutcome {
    trace: InvocationTrace,
    response: EngineResponse,
    parity: ParityReport,
    mismatch_events: Vec<MismatchEvent>,
    shadow_result: Option<EngineResponse>,
    execution_path: ExecutionPath,
    rollback_reason: Option<RollbackReason>,
}

impl InvocationOutcome {
    pub fn trace(&self) -> &InvocationTrace {
        &self.trace
    }

    pub fn response(&self) -> &EngineResponse {
        &self.response
    }

    pub fn parity(&self) -> &ParityReport {
        &self.parity
    }

    pub fn mismatch_events(&self) -> &[MismatchEvent] {
        &self.mismatch_events
    }

    pub fn shadow_result(&self) -> Option<&EngineResponse> {
        self.shadow_result.as_ref()
    }

    pub fn execution_path(&self) -> ExecutionPath {
        self.execution_path
    }

    pub fn rollback_reason(&self) -> Option<RollbackReason> {
        self.rollback_reason
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HostRuntimeError {
    EmptyToolName,
    UnsupportedPrimaryEngine { configured: EngineKind },
    EngineKindMismatch { expected: EngineKind, actual: EngineKind },
    MissingMockResult { engine: EngineKind },
    EngineFailure { engine: EngineKind },
}

impl std::fmt::Display for HostRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyToolName => write!(f, "empty_tool_name"),
            Self::UnsupportedPrimaryEngine { configured } => {
                write!(f, "unsupported_primary_engine_{configured:?}")
            }
            Self::EngineKindMismatch { expected, actual } => {
                write!(f, "engine_kind_mismatch_expected_{expected:?}_actual_{actual:?}")
            }
            Self::MissingMockResult { engine } => {
                write!(f, "missing_mock_result_{engine:?}")
            }
            Self::EngineFailure { engine } => {
                write!(f, "engine_failure_{engine:?}")
            }
        }
    }
}

impl std::error::Error for HostRuntimeError {}

#[derive(Debug, Clone)]
pub struct HostRuntimeBuilder {
    shadow_mode: ShadowMode,
    primary_engine: EngineKind,
    mismatch_policy: MismatchPolicy,
    rollback_switch: RollbackSwitch,
    output_envelope: OutputEnvelope,
    compare_rules: CompareRules,
}

impl HostRuntimeBuilder {
    pub fn shadow_mode(mut self, shadow_mode: ShadowMode) -> Self {
        self.shadow_mode = shadow_mode;
        self
    }

    pub fn primary_engine(mut self, primary_engine: EngineKind) -> Self {
        self.primary_engine = primary_engine;
        self
    }

    pub fn mismatch_policy(mut self, mismatch_policy: MismatchPolicy) -> Self {
        self.mismatch_policy = mismatch_policy;
        self
    }

    pub fn rollback_switch(mut self, rollback_switch: RollbackSwitch) -> Self {
        self.rollback_switch = rollback_switch;
        self
    }

    pub fn output_envelope(mut self, output_envelope: OutputEnvelope) -> Self {
        self.output_envelope = output_envelope;
        self
    }

    pub fn compare_rules(mut self, compare_rules: CompareRules) -> Self {
        self.compare_rules = compare_rules;
        self
    }

    pub fn build_for_test(self) -> HostRuntime {
        HostRuntime {
            shadow_mode: self.shadow_mode,
            primary_engine: self.primary_engine,
            mismatch_policy: self.mismatch_policy,
            output_envelope: self.output_envelope,
            compare_rules: self.compare_rules,
            rollback_state: self.rollback_switch.enabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HostRuntime {
    shadow_mode: ShadowMode,
    primary_engine: EngineKind,
    mismatch_policy: MismatchPolicy,
    output_envelope: OutputEnvelope,
    compare_rules: CompareRules,
    rollback_state: bool,
}

impl HostRuntime {
    pub fn builder() -> HostRuntimeBuilder {
        HostRuntimeBuilder {
            shadow_mode: ShadowMode::DualExecute,
            primary_engine: EngineKind::TypeScript,
            mismatch_policy: MismatchPolicy::LogAndReturnTs,
            rollback_switch: RollbackSwitch::in_memory(false),
            output_envelope: OutputEnvelope::bounded(4096, 4096),
            compare_rules: CompareRules::deterministic("default"),
        }
    }

    pub fn with_rollback_state(mut self, enabled: bool) -> Self {
        self.rollback_state = enabled;
        self
    }

    pub fn set_rollback_state(&mut self, enabled: bool) {
        self.rollback_state = enabled;
    }

    pub fn invoke_with(
        &self,
        call: ToolCall,
        ts_engine: MockEngine,
        wasm_engine: MockEngine,
    ) -> Result<InvocationOutcome, HostRuntimeError> {
        if call.tool_name.is_empty() {
            return Err(HostRuntimeError::EmptyToolName);
        }

        let _argument_count = call.arguments.len();

        if self.primary_engine != EngineKind::TypeScript {
            return Err(HostRuntimeError::UnsupportedPrimaryEngine {
                configured: self.primary_engine,
            });
        }

        let correlation_id =
            call.correlation_id.clone().unwrap_or_else(|| "missing-correlation-id".to_string());

        let ts_response = self.run_engine(EngineKind::TypeScript, ts_engine)?;

        let request_shadow_enabled = call.shadow_hint.unwrap_or(true);
        let runtime_flag_enabled = self.rollback_state;
        let should_run_wasm = self.shadow_mode == ShadowMode::DualExecute
            && !runtime_flag_enabled
            && request_shadow_enabled;

        if should_run_wasm {
            let wasm_response = self.run_engine(EngineKind::Wasm, wasm_engine)?;
            let parity = self.evaluate_parity(&ts_response, &wasm_response);
            let mismatch_events = mismatch_events_from_parity(&parity);
            let trace = InvocationTrace {
                typescript_invocations: 1,
                wasm_invocations: 1,
                typescript_correlation_id: correlation_id.clone(),
                wasm_correlation_id: correlation_id,
            };

            let outcome = InvocationOutcome {
                trace,
                response: ts_response,
                parity,
                mismatch_events,
                shadow_result: Some(wasm_response),
                execution_path: ExecutionPath::ShadowDual,
                rollback_reason: None,
            };

            return Ok(outcome);
        }

        let rollback_reason = if runtime_flag_enabled {
            Some(RollbackReason::RuntimeFlag)
        } else if !request_shadow_enabled {
            Some(RollbackReason::RequestShadowHint)
        } else {
            None
        };

        let execution_path = if runtime_flag_enabled {
            ExecutionPath::TsOnlyRollback
        } else {
            ExecutionPath::TsOnlyRequestHint
        };

        let trace = InvocationTrace {
            typescript_invocations: 1,
            wasm_invocations: 0,
            typescript_correlation_id: correlation_id.clone(),
            wasm_correlation_id: correlation_id,
        };

        let parity = ParityReport {
            semantic_match: true,
            byte_match: true,
            classification: ParityClassification::FullMatch,
            decision: ParityDecision::ReturnTypeScriptRollback,
            rule_version: self.compare_rules.rule_version.clone(),
            deterministic: self.compare_rules.deterministic,
            byte_mismatch: ByteMismatch { first_offset: None },
        };

        Ok(InvocationOutcome {
            trace,
            response: ts_response,
            parity,
            mismatch_events: Vec::new(),
            shadow_result: None,
            execution_path,
            rollback_reason,
        })
    }

    fn run_engine(
        &self,
        expected_kind: EngineKind,
        engine: MockEngine,
    ) -> Result<EngineResponse, HostRuntimeError> {
        if engine.kind != expected_kind {
            return Err(HostRuntimeError::EngineKindMismatch {
                expected: expected_kind,
                actual: engine.kind,
            });
        }

        let result =
            engine.result.ok_or(HostRuntimeError::MissingMockResult { engine: expected_kind })?;

        if !result.success {
            return Err(HostRuntimeError::EngineFailure { engine: expected_kind });
        }

        let (stdout_bytes, stdout_truncated) =
            truncate_bytes(&result.stdout, self.output_envelope.stdout_limit);
        let (stderr_bytes, stderr_truncated) =
            truncate_bytes(&result.stderr, self.output_envelope.stderr_limit);

        Ok(EngineResponse {
            engine: expected_kind,
            body_bytes: result.bytes,
            semantic: result.semantic,
            stdout_bytes,
            stderr_bytes,
            stdout_truncated,
            stderr_truncated,
            output_limit_stdout_bytes: self.output_envelope.stdout_limit,
            output_limit_stderr_bytes: self.output_envelope.stderr_limit,
        })
    }

    fn evaluate_parity(
        &self,
        ts_response: &EngineResponse,
        wasm_response: &EngineResponse,
    ) -> ParityReport {
        let semantic_match = match self.compare_rules.semantic_rule {
            SemanticParity::CanonicalJson => canonical_json_match(
                ts_response.semantic(),
                wasm_response.semantic(),
                &self.compare_rules.ignore_fields,
            ),
        };

        let (byte_match, byte_mismatch) = match self.compare_rules.byte_rule {
            ByteParity::Exact => byte_parity_exact(ts_response.body(), wasm_response.body()),
        };

        let classification = classify_parity(semantic_match, byte_match);
        let decision = decide_parity(self.mismatch_policy, classification);

        ParityReport {
            semantic_match,
            byte_match,
            classification,
            decision,
            rule_version: self.compare_rules.rule_version.clone(),
            deterministic: self.compare_rules.deterministic,
            byte_mismatch,
        }
    }
}

fn truncate_bytes(bytes: &[u8], limit: usize) -> (Vec<u8>, bool) {
    if bytes.len() <= limit {
        return (bytes.to_vec(), false);
    }

    (bytes[..limit].to_vec(), true)
}

fn canonical_json_match(
    left: Option<&SemanticPayload>,
    right: Option<&SemanticPayload>,
    ignore_fields: &BTreeSet<String>,
) -> bool {
    let (Some(left_payload), Some(right_payload)) = (left, right) else {
        return false;
    };

    let mut left_value = left_payload.value.clone();
    let mut right_value = right_payload.value.clone();

    if let Value::Object(left_map) = &mut left_value {
        for field in ignore_fields {
            left_map.remove(field);
        }
    }

    if let Value::Object(right_map) = &mut right_value {
        for field in ignore_fields {
            right_map.remove(field);
        }
    }

    left_value == right_value
}

fn byte_parity_exact(left: &[u8], right: &[u8]) -> (bool, ByteMismatch) {
    if left == right {
        return (true, ByteMismatch { first_offset: None });
    }

    let shared_len = left.len().min(right.len());
    let mut offset = 0usize;

    while offset < shared_len {
        if left[offset] != right[offset] {
            return (false, ByteMismatch { first_offset: Some(offset) });
        }
        offset += 1;
    }

    (false, ByteMismatch { first_offset: Some(shared_len) })
}

fn classify_parity(semantic_match: bool, byte_match: bool) -> ParityClassification {
    if semantic_match && byte_match {
        return ParityClassification::FullMatch;
    }

    if !semantic_match {
        return ParityClassification::SemanticMismatch;
    }

    ParityClassification::ByteMismatch
}

fn decide_parity(
    mismatch_policy: MismatchPolicy,
    classification: ParityClassification,
) -> ParityDecision {
    match mismatch_policy {
        MismatchPolicy::LogAndReturnTs => {
            if classification == ParityClassification::FullMatch {
                ParityDecision::ReturnTypeScript
            } else {
                ParityDecision::ReturnTypeScriptWithLoggedMismatch
            }
        }
    }
}

fn mismatch_events_from_parity(parity: &ParityReport) -> Vec<MismatchEvent> {
    if parity.classification == ParityClassification::FullMatch {
        return Vec::new();
    }

    let semantic_status =
        if parity.semantic_match { ParityStatus::Match } else { ParityStatus::Mismatch };

    let byte_status = if parity.byte_match { ParityStatus::Match } else { ParityStatus::Mismatch };

    vec![MismatchEvent { semantic_status, byte_status }]
}
