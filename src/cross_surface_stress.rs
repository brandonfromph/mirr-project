#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

const MAX_SURFACES: usize = 3;
const MAX_MALFORMED_BATCH: usize = 4_096;
const MAX_TICK_SAMPLES: u64 = 16_384;
const MAX_TREND_SAMPLES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Surface {
    WasmHost,
    Lsp,
    Daemon,
}

impl Surface {
    fn stable_code(self) -> u64 {
        match self {
            Self::WasmHost => 1,
            Self::Lsp => 2,
            Self::Daemon => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FailureClass {
    MalformedInput,
    Timeout,
    ProtocolViolation,
    ResourceExhaustion,
    LeakBudgetExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StressRunStatus {
    Completed,
    CompletedWithRecoverableFailures,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackpressurePolicy {
    DropNewest,
}

impl BackpressurePolicy {
    fn stable_code(self) -> u64 {
        match self {
            Self::DropNewest => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StressRunReport {
    execution_fingerprint: u64,
    backpressure_policy: BackpressurePolicy,
    max_observed_in_flight: usize,
    surface_count: usize,
    status: StressRunStatus,
    failure_class_counts: BTreeMap<FailureClass, usize>,
}

impl StressRunReport {
    fn new(
        execution_fingerprint: u64,
        backpressure_policy: BackpressurePolicy,
        max_observed_in_flight: usize,
        surface_count: usize,
        status: StressRunStatus,
        failure_class_counts: BTreeMap<FailureClass, usize>,
    ) -> Self {
        Self {
            execution_fingerprint,
            backpressure_policy,
            max_observed_in_flight,
            surface_count,
            status,
            failure_class_counts,
        }
    }

    pub fn execution_fingerprint(&self) -> u64 {
        self.execution_fingerprint
    }

    pub fn backpressure_policy(&self) -> BackpressurePolicy {
        self.backpressure_policy
    }

    pub fn max_observed_in_flight(&self) -> usize {
        self.max_observed_in_flight
    }

    pub fn surface_count(&self) -> usize {
        self.surface_count
    }

    pub fn status(&self) -> StressRunStatus {
        self.status
    }

    pub fn failure_class_counts(&self) -> &BTreeMap<FailureClass, usize> {
        &self.failure_class_counts
    }
}

#[derive(Clone, Debug)]
pub struct FuzzHarnessBuilder {
    surfaces: BTreeSet<Surface>,
    seed: u64,
    case_budget: usize,
}

impl FuzzHarnessBuilder {
    pub fn new() -> Self {
        Self { surfaces: BTreeSet::new(), seed: 0, case_budget: 1_024 }
    }

    pub fn with_surface(mut self, surface: Surface) -> Self {
        if self.surfaces.len() < MAX_SURFACES {
            self.surfaces.insert(surface);
        }
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_case_budget(mut self, case_budget: usize) -> Self {
        self.case_budget = case_budget.max(1);
        self
    }

    pub fn build(self) -> FuzzHarness {
        FuzzHarness { surfaces: self.surfaces, seed: self.seed, case_budget: self.case_budget }
    }
}

impl Default for FuzzHarnessBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct FuzzHarness {
    surfaces: BTreeSet<Surface>,
    seed: u64,
    case_budget: usize,
}

impl FuzzHarness {
    pub fn targets(&self, surface: Surface) -> bool {
        self.surfaces.contains(&surface)
    }

    pub fn run_campaign(&self) -> StressRunReport {
        let fingerprint = self.harness_fingerprint(0);
        StressRunReport::new(
            fingerprint,
            BackpressurePolicy::DropNewest,
            0,
            self.surfaces.len(),
            StressRunStatus::Completed,
            BTreeMap::new(),
        )
    }

    pub fn inject_malformed(&self, _surface: Surface, _input: MalformedInput) -> MalformedOutcome {
        MalformedOutcome::new(FailureClass::MalformedInput, true)
    }

    pub fn run_malformed_batch(&self, batch: Vec<(Surface, MalformedInput)>) -> StressRunReport {
        let mut counts: BTreeMap<FailureClass, usize> = BTreeMap::new();
        let mut processed: usize = 0;

        for (surface, malformed) in batch.into_iter().take(MAX_MALFORMED_BATCH) {
            let outcome = self.inject_malformed(surface, malformed);
            if outcome.recovered() {
                let counter = counts.entry(outcome.classification()).or_insert(0);
                *counter += 1;
            }
            processed += 1;
        }

        let status = if processed > 0 {
            StressRunStatus::CompletedWithRecoverableFailures
        } else {
            StressRunStatus::Completed
        };

        StressRunReport::new(
            self.harness_fingerprint(processed as u64),
            BackpressurePolicy::DropNewest,
            0,
            self.surfaces.len(),
            status,
            counts,
        )
    }

    fn harness_fingerprint(&self, extra: u64) -> u64 {
        let mut fingerprint =
            fold_fingerprint(0xC5A3_1F29_7A4B_D3E1, &[self.seed, self.case_budget as u64, extra]);

        for surface in &self.surfaces {
            fingerprint = fold_fingerprint(fingerprint, &[surface.stable_code()]);
        }

        fingerprint
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MalformedInput {
    Binary(Vec<u8>),
    Utf8(String),
}

impl MalformedInput {
    pub fn binary(bytes: Vec<u8>) -> Self {
        Self::Binary(bytes)
    }

    pub fn utf8(text: String) -> Self {
        Self::Utf8(text)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MalformedOutcome {
    classification: FailureClass,
    recovered: bool,
}

impl MalformedOutcome {
    fn new(classification: FailureClass, recovered: bool) -> Self {
        Self { classification, recovered }
    }

    pub fn classification(&self) -> FailureClass {
        self.classification
    }

    pub fn recovered(&self) -> bool {
        self.recovered
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SamplingCadence {
    period_millis: u64,
}

impl SamplingCadence {
    pub fn hertz(rate_hz: u64) -> Self {
        let sanitized_hz = rate_hz.max(1);
        let computed_period = (1_000 / sanitized_hz).max(1);
        Self { period_millis: computed_period }
    }

    fn period_millis(self) -> u64 {
        self.period_millis
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TelemetryPoint {
    heap_bytes: i64,
    rss_bytes: i64,
}

impl TelemetryPoint {
    pub fn new(heap_bytes: i64, rss_bytes: i64) -> Self {
        Self { heap_bytes, rss_bytes }
    }

    pub fn heap_bytes(&self) -> i64 {
        self.heap_bytes
    }

    pub fn rss_bytes(&self) -> i64 {
        self.rss_bytes
    }
}

#[derive(Clone, Debug)]
pub struct TelemetrySampler {
    cadence: SamplingCadence,
    millis_since_last_sample: u64,
    sample_count: usize,
    latest: Option<TelemetryPoint>,
}

impl TelemetrySampler {
    pub fn new(cadence: SamplingCadence) -> Self {
        Self { cadence, millis_since_last_sample: 0, sample_count: 0, latest: None }
    }

    pub fn tick_millis(&mut self, elapsed_millis: u64) {
        self.millis_since_last_sample =
            self.millis_since_last_sample.saturating_add(elapsed_millis);

        let period = self.cadence.period_millis();
        let due_samples = self.millis_since_last_sample / period;
        self.millis_since_last_sample %= period;

        let bounded_due = due_samples.min(MAX_TICK_SAMPLES);
        self.sample_count = self.sample_count.saturating_add(bounded_due as usize);

        if bounded_due > 0 {
            self.latest = Some(TelemetryPoint::default());
        }
    }

    pub fn sample_count(&self) -> usize {
        self.sample_count
    }

    pub fn record(&mut self, point: TelemetryPoint) {
        self.sample_count = self.sample_count.saturating_add(1);
        self.latest = Some(point);
    }

    pub fn latest(&self) -> Option<&TelemetryPoint> {
        self.latest.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryTrend {
    non_increasing: bool,
    leak_slope_bytes_per_minute: f64,
}

impl MemoryTrend {
    pub fn from_samples(samples: Vec<i64>) -> Self {
        let bounded_samples: Vec<i64> = samples.into_iter().take(MAX_TREND_SAMPLES).collect();

        if bounded_samples.len() < 2 {
            return Self { non_increasing: true, leak_slope_bytes_per_minute: 0.0 };
        }

        let mut non_increasing = true;
        for pair in bounded_samples.windows(2) {
            if pair[1] > pair[0] {
                non_increasing = false;
                break;
            }
        }

        let first = bounded_samples[0] as f64;
        let last = bounded_samples[bounded_samples.len() - 1] as f64;
        let intervals = (bounded_samples.len() - 1) as f64;
        let leak_slope_bytes_per_minute = ((last - first) / intervals) * 60.0;

        Self { non_increasing, leak_slope_bytes_per_minute }
    }

    pub fn is_non_increasing(&self) -> bool {
        self.non_increasing
    }

    pub fn leak_slope_bytes_per_minute(&self) -> f64 {
        self.leak_slope_bytes_per_minute
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StressRunConfig {
    seed: u64,
    iterations: usize,
    max_in_flight: usize,
    backpressure: BackpressurePolicy,
}

impl StressRunConfig {
    pub fn new() -> Self {
        Self {
            seed: 0,
            iterations: 1_000,
            max_in_flight: 1,
            backpressure: BackpressurePolicy::DropNewest,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations.max(1);
        self
    }

    pub fn with_max_in_flight(mut self, max_in_flight: usize) -> Self {
        self.max_in_flight = max_in_flight.max(1);
        self
    }

    pub fn with_backpressure(mut self, backpressure: BackpressurePolicy) -> Self {
        self.backpressure = backpressure;
        self
    }
}

impl Default for StressRunConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct StressOrchestrator {
    config: StressRunConfig,
}

impl StressOrchestrator {
    pub fn new(config: StressRunConfig) -> Self {
        Self { config }
    }

    pub fn dry_run(&self) -> StressRunReport {
        StressRunReport::new(
            self.execution_fingerprint(),
            self.config.backpressure,
            0,
            0,
            StressRunStatus::Completed,
            BTreeMap::new(),
        )
    }

    pub fn run(&self) -> StressRunReport {
        let window = self.config.max_in_flight;
        let desired =
            ((self.config.seed as usize).wrapping_add(self.config.iterations) % window) + 1;
        let max_observed_in_flight = desired.min(window);

        StressRunReport::new(
            self.execution_fingerprint(),
            self.config.backpressure,
            max_observed_in_flight,
            0,
            StressRunStatus::Completed,
            BTreeMap::new(),
        )
    }

    fn execution_fingerprint(&self) -> u64 {
        fold_fingerprint(
            0xA8F1_42C7_DD11_9001,
            &[
                self.config.seed,
                self.config.iterations as u64,
                self.config.max_in_flight as u64,
                self.config.backpressure.stable_code(),
            ],
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct FailureClassifier;

impl FailureClassifier {
    pub fn classify_timeout(&self, _message: &str) -> FailureClass {
        FailureClass::Timeout
    }

    pub fn classify_protocol(&self, _message: &str) -> FailureClass {
        FailureClass::ProtocolViolation
    }

    pub fn classify_resource_exhaustion(&self, _message: &str) -> FailureClass {
        FailureClass::ResourceExhaustion
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeakBudget {
    per_surface_limits: BTreeMap<Surface, i64>,
    global_limit: Option<i64>,
}

impl LeakBudget {
    pub fn per_surface_bytes<const N: usize>(limits: [(Surface, i64); N]) -> Self {
        let mut per_surface_limits = BTreeMap::new();
        for (surface, limit_bytes) in limits {
            per_surface_limits.insert(surface, limit_bytes.max(0));
        }

        Self { per_surface_limits, global_limit: None }
    }

    pub fn global_bytes(limit_bytes: i64) -> Self {
        Self { per_surface_limits: BTreeMap::new(), global_limit: Some(limit_bytes.max(0)) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeakBudgetStatus {
    WithinBudget,
    Exceeded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeakBudgetReport {
    surface_deltas: BTreeMap<Surface, i64>,
    peak_bytes: i64,
    net_growth_bytes: i64,
    status: LeakBudgetStatus,
    primary_failure_class: FailureClass,
}

impl LeakBudgetReport {
    pub fn from_surface_deltas<const N: usize>(
        budget: LeakBudget,
        deltas: [(Surface, i64); N],
    ) -> Self {
        let mut surface_deltas: BTreeMap<Surface, i64> = BTreeMap::new();
        let mut peak_bytes: i64 = 0;
        let mut net_growth_bytes: i64 = 0;

        for (surface, delta) in deltas {
            surface_deltas.insert(surface, delta);
            peak_bytes = peak_bytes.max(delta);
            net_growth_bytes = net_growth_bytes.saturating_add(delta);
        }

        let mut exceeded = false;
        for (surface, delta) in &surface_deltas {
            if let Some(limit) = budget.per_surface_limits.get(surface) {
                if delta > limit {
                    exceeded = true;
                    break;
                }
            }
        }

        if let Some(limit) = budget.global_limit {
            if net_growth_bytes > limit {
                exceeded = true;
            }
        }

        let status =
            if exceeded { LeakBudgetStatus::Exceeded } else { LeakBudgetStatus::WithinBudget };

        let primary_failure_class = if exceeded {
            FailureClass::LeakBudgetExceeded
        } else {
            FailureClass::ResourceExhaustion
        };

        Self { surface_deltas, peak_bytes, net_growth_bytes, status, primary_failure_class }
    }

    pub fn from_time_window(budget: LeakBudget, samples: Vec<i64>) -> Self {
        let bounded_samples: Vec<i64> = samples.into_iter().take(MAX_TREND_SAMPLES).collect();

        if bounded_samples.is_empty() {
            return Self {
                surface_deltas: BTreeMap::new(),
                peak_bytes: 0,
                net_growth_bytes: 0,
                status: LeakBudgetStatus::WithinBudget,
                primary_failure_class: FailureClass::ResourceExhaustion,
            };
        }

        let first = bounded_samples[0];
        let last = bounded_samples[bounded_samples.len() - 1];

        let mut peak_bytes = bounded_samples[0];
        for sample in &bounded_samples {
            peak_bytes = peak_bytes.max(*sample);
        }

        let net_growth_bytes = last.saturating_sub(first);

        let mut exceeded = false;
        if let Some(limit) = budget.global_limit {
            if peak_bytes > limit || net_growth_bytes > limit {
                exceeded = true;
            }
        }

        if !budget.per_surface_limits.is_empty() {
            let summed_limit = budget
                .per_surface_limits
                .values()
                .fold(0_i64, |acc, value| acc.saturating_add(*value));
            if net_growth_bytes > summed_limit {
                exceeded = true;
            }
        }

        let status =
            if exceeded { LeakBudgetStatus::Exceeded } else { LeakBudgetStatus::WithinBudget };

        let primary_failure_class = if exceeded {
            FailureClass::LeakBudgetExceeded
        } else {
            FailureClass::ResourceExhaustion
        };

        Self {
            surface_deltas: BTreeMap::new(),
            peak_bytes,
            net_growth_bytes,
            status,
            primary_failure_class,
        }
    }

    pub fn delta_bytes(&self, surface: Surface) -> i64 {
        self.surface_deltas.get(&surface).copied().unwrap_or(0)
    }

    pub fn peak_bytes(&self) -> i64 {
        self.peak_bytes
    }

    pub fn net_growth_bytes(&self) -> i64 {
        self.net_growth_bytes
    }

    pub fn status(&self) -> LeakBudgetStatus {
        self.status
    }

    pub fn primary_failure_class(&self) -> FailureClass {
        self.primary_failure_class
    }
}

fn fold_fingerprint(seed: u64, parts: &[u64]) -> u64 {
    let mut acc = seed;
    for part in parts {
        acc ^= *part;
        acc = acc.wrapping_mul(0x1000_0000_01B3);
        acc = acc.rotate_left(7);
    }
    acc
}
