#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};

// -----------------------------------------------------------------------------
// Shared request type
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonRequest {
    id: u64,
    payload: Vec<u8>,
}

impl DaemonRequest {
    pub fn new(id: u64, payload: Vec<u8>) -> Self {
        Self { id, payload }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

// -----------------------------------------------------------------------------
// Daemon core lifecycle/state ownership
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Stopped,
    Running,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonCoreConfig {
    pub queue_capacity: usize,
}

impl Default for DaemonCoreConfig {
    fn default() -> Self {
        Self { queue_capacity: 1024 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaemonCoreError {
    AlreadyRunning,
    NotRunning,
    QueueBackpressure,
    StateOwnerAlreadyClaimed,
    InvalidStateOwnerTicket,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DaemonState {
    epoch: u64,
}

impl DaemonState {
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn set_epoch(&mut self, epoch: u64) {
        self.epoch = epoch;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateOwnerTicket {
    ticket_id: u64,
}

pub struct DaemonCore {
    config: DaemonCoreConfig,
    lifecycle: LifecycleState,
    queue: VecDeque<DaemonRequest>,
    state: DaemonState,
    owner_ticket: Option<u64>,
    next_ticket: u64,
}

impl DaemonCore {
    pub fn new(config: DaemonCoreConfig) -> Self {
        Self {
            queue: VecDeque::with_capacity(config.queue_capacity),
            config,
            lifecycle: LifecycleState::Stopped,
            state: DaemonState::default(),
            owner_ticket: None,
            next_ticket: 1,
        }
    }

    pub fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle
    }

    pub fn start(&mut self) -> Result<(), DaemonCoreError> {
        if self.lifecycle == LifecycleState::Running {
            return Err(DaemonCoreError::AlreadyRunning);
        }
        self.lifecycle = LifecycleState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), DaemonCoreError> {
        if self.lifecycle == LifecycleState::Stopped {
            return Err(DaemonCoreError::NotRunning);
        }
        self.lifecycle = LifecycleState::Stopped;
        self.queue.clear();
        Ok(())
    }

    pub fn enqueue(&mut self, request: DaemonRequest) -> Result<(), DaemonCoreError> {
        if self.lifecycle != LifecycleState::Running {
            return Err(DaemonCoreError::NotRunning);
        }
        if self.queue.len() >= self.config.queue_capacity {
            return Err(DaemonCoreError::QueueBackpressure);
        }
        self.queue.push_back(request);
        Ok(())
    }

    pub fn queue_depth(&self) -> usize {
        self.queue.len()
    }

    pub fn claim_state_owner(&mut self) -> Result<StateOwnerTicket, DaemonCoreError> {
        if self.owner_ticket.is_some() {
            return Err(DaemonCoreError::StateOwnerAlreadyClaimed);
        }
        let ticket_id = self.next_ticket;
        self.next_ticket = self.next_ticket.wrapping_add(1);
        self.owner_ticket = Some(ticket_id);
        Ok(StateOwnerTicket { ticket_id })
    }

    pub fn mutate_state<F>(
        &mut self,
        ticket: &StateOwnerTicket,
        mutate: F,
    ) -> Result<(), DaemonCoreError>
    where
        F: FnOnce(&mut DaemonState),
    {
        if self.owner_ticket != Some(ticket.ticket_id) {
            return Err(DaemonCoreError::InvalidStateOwnerTicket);
        }
        mutate(&mut self.state);
        Ok(())
    }

    pub fn state_snapshot(&self) -> DaemonState {
        self.state.clone()
    }
}

// -----------------------------------------------------------------------------
// Named pipe endpoint model
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeScope {
    LocalMachine,
    RemoteMachine(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipeEndpointConfig {
    path: String,
    scope: PipeScope,
    exclusive: bool,
}

impl PipeEndpointConfig {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into(), scope: PipeScope::LocalMachine, exclusive: false }
    }

    pub fn scope(mut self, scope: PipeScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeError {
    NonCanonicalPath,
    RemoteScopeUnsupported,
    ExclusiveInUse,
    AlreadyDisconnected,
}

pub struct NamedPipeEndpoint {
    inner: Arc<PipeEndpointInner>,
}

struct PipeEndpointInner {
    identity: u64,
    exclusive: bool,
    active_clients: Mutex<usize>,
}

impl NamedPipeEndpoint {
    pub fn bind(config: PipeEndpointConfig) -> Result<Self, PipeError> {
        match config.scope {
            PipeScope::LocalMachine => {}
            PipeScope::RemoteMachine(_) => return Err(PipeError::RemoteScopeUnsupported),
        }

        if !is_canonical_pipe_path(&config.path) {
            return Err(PipeError::NonCanonicalPath);
        }

        let identity = stable_identity(&config.path);
        Ok(Self {
            inner: Arc::new(PipeEndpointInner {
                identity,
                exclusive: config.exclusive,
                active_clients: Mutex::new(0),
            }),
        })
    }

    pub fn identity(&self) -> u64 {
        self.inner.identity
    }

    pub fn accept_client(&self) -> Result<NamedPipeClient, PipeError> {
        self.connect_impl()
    }

    pub fn accept_client_nonblocking(&self) -> Result<NamedPipeClient, PipeError> {
        self.connect_impl()
    }

    pub fn connect(&self) -> Result<NamedPipeClient, PipeError> {
        self.connect_impl()
    }

    fn connect_impl(&self) -> Result<NamedPipeClient, PipeError> {
        let mut active_clients = lock_unpoisoned(&self.inner.active_clients);
        if self.inner.exclusive && *active_clients > 0 {
            return Err(PipeError::ExclusiveInUse);
        }
        *active_clients = active_clients.saturating_add(1);
        Ok(NamedPipeClient::new(Arc::clone(&self.inner)))
    }
}

pub struct NamedPipeClient {
    endpoint: Arc<PipeEndpointInner>,
    connected: Mutex<bool>,
}

impl NamedPipeClient {
    fn new(endpoint: Arc<PipeEndpointInner>) -> Self {
        Self { endpoint, connected: Mutex::new(true) }
    }

    pub fn disconnect(&self) -> Result<(), PipeError> {
        let mut connected = lock_unpoisoned(&self.connected);
        if !*connected {
            return Err(PipeError::AlreadyDisconnected);
        }
        *connected = false;
        drop(connected);

        let mut active_clients = lock_unpoisoned(&self.endpoint.active_clients);
        if *active_clients > 0 {
            *active_clients -= 1;
        }
        Ok(())
    }
}

impl Drop for NamedPipeClient {
    fn drop(&mut self) {
        let mut connected = lock_unpoisoned(&self.connected);
        if !*connected {
            return;
        }
        *connected = false;
        drop(connected);

        let mut active_clients = lock_unpoisoned(&self.endpoint.active_clients);
        if *active_clients > 0 {
            *active_clients -= 1;
        }
    }
}

fn is_canonical_pipe_path(path: &str) -> bool {
    const PREFIX: &str = r"\\.\pipe\";

    if !path.starts_with(PREFIX) {
        return false;
    }

    let suffix = &path[PREFIX.len()..];
    if suffix.is_empty() {
        return false;
    }

    !suffix.contains('\\') && !suffix.contains('/')
}

fn stable_identity(path: &str) -> u64 {
    // Fixed FNV-1a 64-bit for deterministic endpoint identities.
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in path.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// -----------------------------------------------------------------------------
// Deterministic queueing primitives
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueConfig {
    capacity: usize,
}

impl QueueConfig {
    pub fn bounded(capacity: usize) -> Self {
        Self { capacity }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueError {
    Backpressure,
    TimeoutAtTick(u64),
    Cancelled,
    CancelledAtTick(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeout {
    ticks: u64,
}

impl Timeout {
    pub fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }
}

#[derive(Debug, Clone)]
pub struct DeterministicClock {
    tick: Arc<Mutex<u64>>,
}

impl DeterministicClock {
    pub fn from_tick(tick: u64) -> Self {
        Self { tick: Arc::new(Mutex::new(tick)) }
    }

    pub fn now_tick(&self) -> u64 {
        *lock_unpoisoned(&self.tick)
    }

    fn advance_to(&self, tick: u64) {
        let mut current = lock_unpoisoned(&self.tick);
        if tick > *current {
            *current = tick;
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancellationToken {
    state: Arc<Mutex<CancellationState>>,
}

#[derive(Debug, Clone, Copy)]
struct CancellationState {
    cancelled_immediately: bool,
    cancel_at_tick: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CancellationMode {
    None,
    Immediate,
    Scheduled(u64),
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(CancellationState {
                cancelled_immediately: false,
                cancel_at_tick: None,
            })),
        }
    }

    pub fn cancel(&self) {
        let mut state = lock_unpoisoned(&self.state);
        state.cancelled_immediately = true;
    }

    pub fn cancel_at_tick(tick: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(CancellationState {
                cancelled_immediately: false,
                cancel_at_tick: Some(tick),
            })),
        }
    }

    fn mode(&self) -> CancellationMode {
        let state = lock_unpoisoned(&self.state);
        if state.cancelled_immediately {
            CancellationMode::Immediate
        } else if let Some(tick) = state.cancel_at_tick {
            CancellationMode::Scheduled(tick)
        } else {
            CancellationMode::None
        }
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueMetrics {
    pub depth: usize,
    pub rejected: u64,
}

pub struct MpmcQueue {
    inner: Arc<QueueInner>,
}

struct QueueInner {
    config: QueueConfig,
    state: Mutex<QueueState>,
    clock: DeterministicClock,
}

#[derive(Debug)]
struct QueueState {
    high: VecDeque<DaemonRequest>,
    normal: VecDeque<DaemonRequest>,
    low: VecDeque<DaemonRequest>,
    rejected: u64,
}

impl QueueState {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            high: VecDeque::with_capacity(capacity),
            normal: VecDeque::with_capacity(capacity),
            low: VecDeque::with_capacity(capacity),
            rejected: 0,
        }
    }

    fn depth(&self) -> usize {
        self.high.len().saturating_add(self.normal.len()).saturating_add(self.low.len())
    }

    fn push(&mut self, request: DaemonRequest, priority: RequestPriority) {
        match priority {
            RequestPriority::High => self.high.push_back(request),
            RequestPriority::Normal => self.normal.push_back(request),
            RequestPriority::Low => self.low.push_back(request),
        }
    }

    fn pop(&mut self) -> Option<DaemonRequest> {
        if let Some(request) = self.high.pop_front() {
            return Some(request);
        }
        if let Some(request) = self.normal.pop_front() {
            return Some(request);
        }
        self.low.pop_front()
    }
}

impl MpmcQueue {
    pub fn new(config: QueueConfig) -> Self {
        Self::with_clock(config, DeterministicClock::from_tick(0))
    }

    pub fn with_clock(config: QueueConfig, clock: DeterministicClock) -> Self {
        let state = QueueState::with_capacity(config.capacity);
        Self { inner: Arc::new(QueueInner { config, state: Mutex::new(state), clock }) }
    }

    pub fn producer(&self, producer_id: &str) -> QueueProducer {
        QueueProducer { queue: self.clone(), producer_id: producer_id.to_string() }
    }

    pub fn enqueue_with_priority(
        &self,
        request: DaemonRequest,
        priority: RequestPriority,
    ) -> Result<(), QueueError> {
        let mut state = lock_unpoisoned(&self.inner.state);
        if state.depth() >= self.inner.config.capacity {
            state.rejected = state.rejected.saturating_add(1);
            return Err(QueueError::Backpressure);
        }
        state.push(request, priority);
        Ok(())
    }

    pub fn try_enqueue(&self, request: DaemonRequest) -> Result<(), QueueError> {
        self.enqueue_with_priority(request, RequestPriority::Normal)
    }

    pub fn dequeue(&self, timeout: Timeout) -> Result<DaemonRequest, QueueError> {
        self.dequeue_internal(timeout, None)
    }

    pub fn dequeue_with_cancellation(
        &self,
        timeout: Timeout,
        cancellation: &CancellationToken,
    ) -> Result<DaemonRequest, QueueError> {
        self.dequeue_internal(timeout, Some(cancellation))
    }

    pub fn metrics(&self) -> QueueMetrics {
        let state = lock_unpoisoned(&self.inner.state);
        QueueMetrics { depth: state.depth(), rejected: state.rejected }
    }

    fn dequeue_internal(
        &self,
        timeout: Timeout,
        cancellation: Option<&CancellationToken>,
    ) -> Result<DaemonRequest, QueueError> {
        if let Some(token) = cancellation {
            if token.mode() == CancellationMode::Immediate {
                return Err(QueueError::Cancelled);
            }
        }

        {
            let mut state = lock_unpoisoned(&self.inner.state);
            if let Some(request) = state.pop() {
                return Ok(request);
            }
        }

        let now_tick = self.inner.clock.now_tick();
        let deadline_tick = now_tick.saturating_add(timeout.ticks);

        if let Some(token) = cancellation {
            match token.mode() {
                CancellationMode::None => {}
                CancellationMode::Immediate => return Err(QueueError::Cancelled),
                CancellationMode::Scheduled(cancel_tick) => {
                    if cancel_tick <= deadline_tick {
                        let effective_tick =
                            if cancel_tick < now_tick { now_tick } else { cancel_tick };
                        self.inner.clock.advance_to(effective_tick);
                        return Err(QueueError::CancelledAtTick(effective_tick));
                    }
                }
            }
        }

        self.inner.clock.advance_to(deadline_tick);
        Err(QueueError::TimeoutAtTick(deadline_tick))
    }
}

impl Clone for MpmcQueue {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

pub struct QueueProducer {
    queue: MpmcQueue,
    producer_id: String,
}

impl QueueProducer {
    pub fn id(&self) -> &str {
        &self.producer_id
    }

    pub fn enqueue(&self, request: DaemonRequest) -> Result<(), QueueError> {
        self.queue.enqueue_with_priority(request, RequestPriority::Normal)
    }
}

fn lock_unpoisoned<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}
