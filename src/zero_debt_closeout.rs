#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

const MAX_ROLLBACK_STEPS: usize = 256;
const MAX_CANONICAL_SCOPES: usize = 1024;
const MAX_CANONICAL_STRATEGIES: usize = 2048;
const CANONICAL_MAGIC: &[u8] = b"ZDCLOSEOUT1";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnforcementWindow {
    pub required_samples: usize,
    pub required_ratio: f64,
}

impl EnforcementWindow {
    pub fn new(required_samples: usize, required_ratio: f64) -> Self {
        let bounded_ratio =
            if required_ratio.is_finite() { required_ratio.clamp(0.0, 1.0) } else { 1.0 };

        Self { required_samples, required_ratio: bounded_ratio }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShadowSample {
    pub epoch: u64,
    pub parity_match: bool,
}

impl ShadowSample {
    pub fn parity_match(epoch: u64) -> Self {
        Self { epoch, parity_match: true }
    }

    pub fn parity_mismatch(epoch: u64) -> Self {
        Self { epoch, parity_match: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CutoverDecision {
    RemainInShadow,
    CutoverAt { epoch: u64, rollback_plan_id: String },
}

#[derive(Debug, Clone)]
pub struct ShadowModeCutoverPlanner {
    window: EnforcementWindow,
    stable_samples: usize,
    most_recent_epoch: Option<u64>,
}

impl ShadowModeCutoverPlanner {
    pub fn new(window: EnforcementWindow) -> Self {
        Self { window, stable_samples: 0, most_recent_epoch: None }
    }

    pub fn empty_ready_state() -> Self {
        Self::new(EnforcementWindow::new(0, 1.0))
    }

    pub fn record_shadow_sample(&mut self, sample: ShadowSample) {
        self.most_recent_epoch = Some(sample.epoch);
        if sample.parity_match {
            self.stable_samples = self.stable_samples.saturating_add(1);
        } else {
            self.stable_samples = 0;
        }
    }

    pub fn evaluate_cutover(&self, epoch: u64) -> CutoverDecision {
        if self.is_ready_for_cutover() {
            return CutoverDecision::CutoverAt {
                epoch,
                rollback_plan_id: self.rollback_plan_id(epoch),
            };
        }

        CutoverDecision::RemainInShadow
    }

    fn is_ready_for_cutover(&self) -> bool {
        if self.window.required_samples == 0 {
            return true;
        }
        if self.stable_samples < self.window.required_samples {
            return false;
        }

        1.0 >= self.window.required_ratio
    }

    fn rollback_plan_id(&self, epoch: u64) -> String {
        let ratio_ppm = (self.window.required_ratio * 1_000_000.0).round() as u64;
        let observed_epoch = self.most_recent_epoch.unwrap_or(epoch);
        format!(
            "rb-{epoch}-s{}-r{ratio_ppm}-n{}-o{observed_epoch}",
            self.window.required_samples, self.stable_samples
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteResolution {
    Allowed { route: String, status_code: u16, detail: String },
    Disabled { route: String, status_code: u16, detail: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteAuditEvent {
    pub route: String,
    pub epoch: u64,
    pub cutover_active: bool,
    pub resolution: RouteResolution,
}

#[derive(Debug, Clone)]
struct TemporaryReenableGrant {
    ttl_epochs: u32,
    incident_id: String,
    activation_epoch: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct CompatibilityRouteDisablementContract {
    disable_on_cutover: bool,
    disabled_status_code: u16,
    legacy_routes: BTreeSet<String>,
    temporary_reenables: RefCell<BTreeMap<String, TemporaryReenableGrant>>,
    audit_events: RefCell<Vec<RouteAuditEvent>>,
}

impl CompatibilityRouteDisablementContract {
    pub fn strict_default() -> Self {
        Self {
            disable_on_cutover: true,
            disabled_status_code: 410,
            legacy_routes: BTreeSet::new(),
            temporary_reenables: RefCell::new(BTreeMap::new()),
            audit_events: RefCell::new(Vec::new()),
        }
    }

    pub fn with_legacy_route(mut self, route: &str) -> Self {
        if !route.trim().is_empty() {
            self.legacy_routes.insert(route.to_string());
        }
        self
    }

    pub fn disable_on_cutover(mut self) -> Self {
        self.disable_on_cutover = true;
        self
    }

    pub fn request_temporary_reenable(
        &mut self,
        route: &str,
        ttl_epochs: u32,
        incident_id: &str,
    ) -> Result<(), String> {
        if ttl_epochs == u32::MAX {
            return Err("temporary re-enable ttl must be bounded".to_string());
        }
        if !self.legacy_routes.contains(route) {
            return Err(format!("legacy route '{route}' is not registered"));
        }
        if incident_id.trim().is_empty() {
            return Err("incident id for temporary re-enable must be non-empty".to_string());
        }

        self.temporary_reenables.borrow_mut().insert(
            route.to_string(),
            TemporaryReenableGrant {
                ttl_epochs,
                incident_id: incident_id.to_string(),
                activation_epoch: None,
            },
        );
        Ok(())
    }

    pub fn resolve(&self, route: &str, cutover_active: bool, epoch: u64) -> RouteResolution {
        let resolution =
            if self.legacy_routes.contains(route) && cutover_active && self.disable_on_cutover {
                if let Some(incident_id) = self.active_temporary_reenable(route, epoch) {
                    RouteResolution::Allowed {
                        route: route.to_string(),
                        status_code: 200,
                        detail: format!("temporary re-enable via {incident_id}"),
                    }
                } else {
                    RouteResolution::Disabled {
                        route: route.to_string(),
                        status_code: self.disabled_status_code,
                        detail: "legacy route disabled after cutover".to_string(),
                    }
                }
            } else {
                RouteResolution::Allowed {
                    route: route.to_string(),
                    status_code: 200,
                    detail: "route allowed".to_string(),
                }
            };

        self.audit_events.borrow_mut().push(RouteAuditEvent {
            route: route.to_string(),
            epoch,
            cutover_active,
            resolution: resolution.clone(),
        });

        resolution
    }

    pub fn route_audit(&self, route: &str) -> Vec<RouteAuditEvent> {
        self.audit_events.borrow().iter().filter(|event| event.route == route).cloned().collect()
    }

    fn active_temporary_reenable(&self, route: &str, epoch: u64) -> Option<String> {
        let mut grants = self.temporary_reenables.borrow_mut();
        let grant = grants.get_mut(route)?;

        let start_epoch = match grant.activation_epoch {
            Some(value) => value,
            None => {
                grant.activation_epoch = Some(epoch);
                epoch
            }
        };

        let elapsed = epoch.saturating_sub(start_epoch);
        if elapsed < u64::from(grant.ttl_epochs) {
            return Some(grant.incident_id.clone());
        }

        grants.remove(route);
        None
    }

    fn has_legacy_route(&self, route: &str) -> bool {
        self.legacy_routes.contains(route)
    }

    fn disables_on_cutover(&self) -> bool {
        self.disable_on_cutover
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyRoute {
    pub removed_route: String,
    pub replacement_route: Option<String>,
}

impl LegacyRoute {
    pub fn new(removed_route: &str, replacement_route: &str) -> Self {
        Self {
            removed_route: removed_route.to_string(),
            replacement_route: Some(replacement_route.to_string()),
        }
    }

    pub fn without_replacement(removed_route: &str) -> Self {
        Self { removed_route: removed_route.to_string(), replacement_route: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalMode {
    Irreversible,
}

#[derive(Debug, Clone)]
pub struct LegacyPathRemovalManifest {
    routes: BTreeMap<String, String>,
    frozen_mode: Option<RemovalMode>,
}

impl LegacyPathRemovalManifest {
    pub fn from_routes(routes: Vec<LegacyRoute>) -> Result<Self, String> {
        let mut manifest = Self::empty();
        for route in routes {
            manifest.add_route(route)?;
        }
        Ok(manifest)
    }

    pub fn empty() -> Self {
        Self { routes: BTreeMap::new(), frozen_mode: None }
    }

    pub fn add_route(&mut self, route: LegacyRoute) -> Result<(), String> {
        if self.frozen_mode.is_some() {
            return Err("manifest is frozen and cannot accept new routes".to_string());
        }

        let LegacyRoute { removed_route, replacement_route } = route;

        let removed = removed_route.trim();
        if removed.is_empty() {
            return Err("removed legacy route path must be non-empty".to_string());
        }

        let replacement = replacement_route
            .ok_or_else(|| format!("removed route '{removed}' must define a replacement route"))?;
        if replacement.trim().is_empty() {
            return Err(format!("removed route '{removed}' must define a replacement route"));
        }

        if self.routes.contains_key(removed) {
            return Err(format!("duplicate removed route '{removed}'"));
        }

        self.routes.insert(removed.to_string(), replacement.to_string());
        Ok(())
    }

    pub fn freeze(&mut self, mode: RemovalMode) -> Result<(), String> {
        if self.frozen_mode.is_some() {
            return Err("manifest is already frozen".to_string());
        }

        self.frozen_mode = Some(mode);
        Ok(())
    }

    pub fn ordered_removed_routes(&self) -> Vec<String> {
        self.routes.keys().cloned().collect()
    }

    fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZeroDebtInvariant {
    NoWrapperFunctions,
    NoDeprecatedAliases,
    NoBackwardCompatShims,
}

#[derive(Debug, Clone)]
pub struct ZeroDebtCloseoutReport {
    proposal_id: String,
    planner: ShadowModeCutoverPlanner,
    contract: CompatibilityRouteDisablementContract,
    manifest: LegacyPathRemovalManifest,
    invariants: BTreeMap<ZeroDebtInvariant, bool>,
    debt_score: u32,
    section_evidence_ids: BTreeMap<String, String>,
}

impl ZeroDebtCloseoutReport {
    pub fn new(proposal_id: &str) -> Self {
        Self::from_components(
            proposal_id,
            ShadowModeCutoverPlanner::empty_ready_state(),
            CompatibilityRouteDisablementContract::strict_default(),
            LegacyPathRemovalManifest::empty(),
        )
    }

    pub fn from_components(
        proposal_id: &str,
        planner: ShadowModeCutoverPlanner,
        contract: CompatibilityRouteDisablementContract,
        manifest: LegacyPathRemovalManifest,
    ) -> Self {
        Self {
            proposal_id: proposal_id.to_string(),
            planner,
            contract,
            manifest,
            invariants: BTreeMap::new(),
            debt_score: 0,
            section_evidence_ids: BTreeMap::new(),
        }
    }

    pub fn record_invariant(&mut self, invariant: ZeroDebtInvariant, satisfied: bool) {
        self.invariants.insert(invariant, satisfied);
    }

    pub fn is_closeout_ready(&self) -> bool {
        Self::required_invariants()
            .iter()
            .all(|invariant| self.invariants.get(invariant).copied().unwrap_or(false))
            && self.debt_score == 0
            && self.section_evidence_ids.values().all(|value| !value.trim().is_empty())
            && self.validate_component_alignment().is_ok()
    }

    pub fn validate_component_alignment(&self) -> Result<(), String> {
        if self.manifest.is_empty() {
            return Ok(());
        }

        if !self.contract.disables_on_cutover() {
            return Err(
                "compatibility contract must disable legacy routes when manifest removes paths"
                    .to_string(),
            );
        }

        if self.planner.window.required_samples == 0 {
            return Err("planner must use a non-empty enforcement window for non-empty manifest"
                .to_string());
        }

        for removed_route in self.manifest.ordered_removed_routes() {
            if !self.contract.has_legacy_route(&removed_route) {
                return Err(format!(
                    "manifest route '{removed_route}' missing in compatibility contract"
                ));
            }
        }

        Ok(())
    }

    pub fn set_debt_score(&mut self, debt_score: u32) {
        self.debt_score = debt_score;
    }

    pub fn finalize(&self) -> Result<(), String> {
        if self.debt_score > 0 {
            return Err(format!(
                "proposal '{}' cannot finalize with debt score {}",
                self.proposal_id, self.debt_score
            ));
        }
        if !self.is_closeout_ready() {
            return Err(format!("proposal '{}' closeout report is not ready", self.proposal_id));
        }

        Ok(())
    }

    pub fn with_section_evidence(mut self, section: &str, evidence_id: &str) -> Self {
        if !section.trim().is_empty() && !evidence_id.trim().is_empty() {
            self.section_evidence_ids.insert(section.to_string(), evidence_id.to_string());
        }
        self
    }

    pub fn section_evidence_ids(&self) -> &BTreeMap<String, String> {
        &self.section_evidence_ids
    }

    fn required_invariants() -> [ZeroDebtInvariant; 3] {
        [
            ZeroDebtInvariant::NoWrapperFunctions,
            ZeroDebtInvariant::NoDeprecatedAliases,
            ZeroDebtInvariant::NoBackwardCompatShims,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredScope {
    pub scope: String,
    pub owner: String,
    pub reason: String,
}

impl DeferredScope {
    pub fn new(scope: &str, owner: &str, reason: &str) -> Result<Self, String> {
        if scope.trim().is_empty() {
            return Err("deferred scope name must be non-empty".to_string());
        }
        if owner.trim().is_empty() || reason.trim().is_empty() {
            return Err("deferred scope owner and reason must be non-empty".to_string());
        }

        Ok(Self { scope: scope.to_string(), owner: owner.to_string(), reason: reason.to_string() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackMetadata {
    pub rollback_plan_id: String,
    pub steps: Vec<String>,
    pub strategy_by_scope: BTreeMap<String, String>,
}

impl RollbackMetadata {
    pub fn new(rollback_plan_id: &str, steps: Vec<String>) -> Result<Self, String> {
        if rollback_plan_id.trim().is_empty() {
            return Err("rollback plan id must be non-empty".to_string());
        }
        if steps.len() > MAX_ROLLBACK_STEPS {
            return Err(format!("rollback metadata cannot exceed {MAX_ROLLBACK_STEPS} steps"));
        }
        if steps.iter().any(|step| step.trim().is_empty()) {
            return Err("rollback step identifiers must be non-empty".to_string());
        }

        Ok(Self {
            rollback_plan_id: rollback_plan_id.to_string(),
            steps,
            strategy_by_scope: BTreeMap::new(),
        })
    }

    pub fn with_strategy_for(mut self, scope: &str, strategy: &str) -> Self {
        if !scope.trim().is_empty() && !strategy.trim().is_empty() {
            self.strategy_by_scope.insert(scope.to_string(), strategy.to_string());
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredScopeRollbackMetadata {
    deferred_scopes: Vec<DeferredScope>,
    rollback: RollbackMetadata,
}

impl DeferredScopeRollbackMetadata {
    pub fn new(
        mut deferred_scopes: Vec<DeferredScope>,
        rollback: RollbackMetadata,
    ) -> Result<Self, String> {
        deferred_scopes.sort_by(|left, right| {
            left.scope
                .cmp(&right.scope)
                .then(left.owner.cmp(&right.owner))
                .then(left.reason.cmp(&right.reason))
        });

        let mut seen = BTreeSet::new();
        for scope in &deferred_scopes {
            if !seen.insert(scope.scope.clone()) {
                return Err(format!("duplicate deferred scope '{}'", scope.scope));
            }
        }

        Ok(Self { deferred_scopes, rollback })
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(CANONICAL_MAGIC);

        push_u32(&mut bytes, to_u32(self.deferred_scopes.len()));
        for scope in &self.deferred_scopes {
            push_string(&mut bytes, &scope.scope);
            push_string(&mut bytes, &scope.owner);
            push_string(&mut bytes, &scope.reason);
        }

        push_string(&mut bytes, &self.rollback.rollback_plan_id);
        push_u32(&mut bytes, to_u32(self.rollback.steps.len()));
        for step in &self.rollback.steps {
            push_string(&mut bytes, step);
        }

        push_u32(&mut bytes, to_u32(self.rollback.strategy_by_scope.len()));
        for (scope, strategy) in &self.rollback.strategy_by_scope {
            push_string(&mut bytes, scope);
            push_string(&mut bytes, strategy);
        }

        bytes
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, String> {
        if !bytes.starts_with(CANONICAL_MAGIC) {
            return Err("canonical bytes header mismatch".to_string());
        }

        let mut cursor = CANONICAL_MAGIC.len();

        let scope_count = to_usize(read_u32(bytes, &mut cursor)?)?;
        if scope_count > MAX_CANONICAL_SCOPES {
            return Err(format!("canonical scope count exceeds limit ({MAX_CANONICAL_SCOPES})"));
        }

        let mut deferred_scopes = Vec::with_capacity(scope_count);
        for _ in 0..scope_count {
            let scope = read_string(bytes, &mut cursor)?;
            let owner = read_string(bytes, &mut cursor)?;
            let reason = read_string(bytes, &mut cursor)?;
            deferred_scopes.push(DeferredScope::new(&scope, &owner, &reason)?);
        }

        let rollback_plan_id = read_string(bytes, &mut cursor)?;
        let step_count = to_usize(read_u32(bytes, &mut cursor)?)?;
        if step_count > MAX_ROLLBACK_STEPS {
            return Err(format!("canonical step count exceeds limit ({MAX_ROLLBACK_STEPS})"));
        }

        let mut steps = Vec::with_capacity(step_count);
        for _ in 0..step_count {
            steps.push(read_string(bytes, &mut cursor)?);
        }

        let strategy_count = to_usize(read_u32(bytes, &mut cursor)?)?;
        if strategy_count > MAX_CANONICAL_STRATEGIES {
            return Err(format!(
                "canonical strategy count exceeds limit ({MAX_CANONICAL_STRATEGIES})"
            ));
        }

        let mut rollback = RollbackMetadata::new(&rollback_plan_id, steps)?;
        for _ in 0..strategy_count {
            let scope = read_string(bytes, &mut cursor)?;
            let strategy = read_string(bytes, &mut cursor)?;
            rollback = rollback.with_strategy_for(&scope, &strategy);
        }

        if cursor != bytes.len() {
            return Err("canonical bytes contain trailing data".to_string());
        }

        Self::new(deferred_scopes, rollback)
    }

    pub fn deterministic_hash(&self) -> String {
        const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
        const FNV_PRIME: u64 = 1_099_511_628_211;

        let mut hash = FNV_OFFSET_BASIS;
        for byte in self.to_canonical_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        format!("{hash:016x}")
    }

    pub fn validate_one_to_one_scope_to_strategy(&self) -> Result<(), String> {
        let mut scope_set = BTreeSet::new();
        for scope in &self.deferred_scopes {
            scope_set.insert(scope.scope.clone());
        }

        let mut strategy_scope_set = BTreeSet::new();
        for (scope, strategy) in &self.rollback.strategy_by_scope {
            if strategy.trim().is_empty() {
                return Err(format!("rollback strategy for scope '{scope}' is empty"));
            }
            strategy_scope_set.insert(scope.clone());
        }

        if scope_set == strategy_scope_set {
            return Ok(());
        }

        let missing: Vec<String> = scope_set.difference(&strategy_scope_set).cloned().collect();
        let extra: Vec<String> = strategy_scope_set.difference(&scope_set).cloned().collect();

        Err(format!(
            "scope/strategy mapping mismatch (missing: [{}], extra: [{}])",
            missing.join(", "),
            extra.join(", ")
        ))
    }
}

fn to_u32(value: usize) -> u32 {
    if value > u32::MAX as usize {
        u32::MAX
    } else {
        value as u32
    }
}

fn to_usize(value: u32) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| "length does not fit into usize".to_string())
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_string(bytes: &mut Vec<u8>, value: &str) {
    let value_bytes = value.as_bytes();
    let capped_len = value_bytes.len().min(u32::MAX as usize);
    push_u32(bytes, to_u32(capped_len));
    bytes.extend_from_slice(&value_bytes[..capped_len]);
}

fn read_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, String> {
    if bytes.len().saturating_sub(*cursor) < 4 {
        return Err("canonical bytes truncated while reading u32".to_string());
    }

    let mut raw = [0_u8; 4];
    raw.copy_from_slice(&bytes[*cursor..*cursor + 4]);
    *cursor += 4;
    Ok(u32::from_le_bytes(raw))
}

fn read_string(bytes: &[u8], cursor: &mut usize) -> Result<String, String> {
    let len = to_usize(read_u32(bytes, cursor)?)?;
    if bytes.len().saturating_sub(*cursor) < len {
        return Err("canonical bytes truncated while reading string".to_string());
    }

    let slice = &bytes[*cursor..*cursor + len];
    *cursor += len;
    let value = std::str::from_utf8(slice)
        .map_err(|_| "canonical bytes contain invalid utf-8".to_string())?;
    Ok(value.to_string())
}
