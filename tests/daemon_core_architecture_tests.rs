#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::sync::Arc;
use std::thread;

use mirrc::mirr_daemon::{
    CancellationToken, DaemonCore, DaemonCoreConfig, DaemonCoreError, DaemonRequest,
    DeterministicClock, LifecycleState, MpmcQueue, NamedPipeEndpoint, PipeEndpointConfig,
    PipeError, PipeScope, QueueConfig, QueueError, RequestPriority, Timeout,
};

fn request(id: u64) -> DaemonRequest {
    DaemonRequest::new(id, vec![id as u8; 8])
}

#[test]
fn daemon_core_starts_in_stopped_state() {
    let core = DaemonCore::new(DaemonCoreConfig::default());
    assert_eq!(core.lifecycle_state(), LifecycleState::Stopped);
}

#[test]
fn daemon_core_start_transitions_to_running_once() {
    let mut core = DaemonCore::new(DaemonCoreConfig::default());
    core.start().expect("start should succeed from stopped state");
    assert_eq!(core.lifecycle_state(), LifecycleState::Running);
}

#[test]
fn daemon_core_rejects_second_start_while_running() {
    let mut core = DaemonCore::new(DaemonCoreConfig::default());
    core.start().expect("first start should succeed");
    let second_start = core.start();
    assert!(matches!(second_start, Err(DaemonCoreError::AlreadyRunning)));
}

#[test]
fn daemon_core_stop_transitions_to_stopped_and_drains_queue() {
    let mut core = DaemonCore::new(DaemonCoreConfig::default());
    core.start().expect("start should succeed");
    core.enqueue(request(1)).expect("enqueue should succeed");
    core.enqueue(request(2)).expect("enqueue should succeed");
    core.stop().expect("stop should succeed");
    assert_eq!(core.lifecycle_state(), LifecycleState::Stopped);
    assert_eq!(core.queue_depth(), 0);
}

#[test]
fn daemon_core_enforces_single_owner_mutation_ticket() {
    let mut core = DaemonCore::new(DaemonCoreConfig::default());
    let ticket = core.claim_state_owner().expect("first owner claim should succeed");
    assert!(core.claim_state_owner().is_err());
    core.mutate_state(&ticket, |state| {
        state.set_epoch(7);
    })
    .expect("state mutation with owner ticket should succeed");
    assert_eq!(core.state_snapshot().epoch(), 7);
}

#[test]
fn named_pipe_endpoint_requires_local_machine_scope() {
    let config = PipeEndpointConfig::new(r"\\.\pipe\mirr-daemon")
        .scope(PipeScope::RemoteMachine("external-host".to_string()));
    assert!(NamedPipeEndpoint::bind(config).is_err());
}

#[test]
fn named_pipe_endpoint_rejects_non_canonical_pipe_name() {
    let config = PipeEndpointConfig::new("mirr-daemon").scope(PipeScope::LocalMachine);
    let bind_attempt = NamedPipeEndpoint::bind(config);
    assert!(matches!(bind_attempt, Err(PipeError::NonCanonicalPath)));
}

#[test]
fn named_pipe_endpoint_allows_only_one_active_client_in_exclusive_mode() {
    let endpoint = NamedPipeEndpoint::bind(
        PipeEndpointConfig::new(r"\\.\pipe\mirr-daemon")
            .scope(PipeScope::LocalMachine)
            .exclusive(true),
    )
    .expect("bind should succeed with canonical local endpoint");

    let _client_a = endpoint.accept_client().expect("first client should connect");
    let second = endpoint.accept_client_nonblocking();
    assert!(matches!(second, Err(PipeError::ExclusiveInUse)));
}

#[test]
fn named_pipe_endpoint_reconnect_keeps_stable_identity() {
    let endpoint = NamedPipeEndpoint::bind(
        PipeEndpointConfig::new(r"\\.\pipe\mirr-daemon").scope(PipeScope::LocalMachine),
    )
    .expect("bind should succeed");

    let identity_before = endpoint.identity();
    let client = endpoint.connect().expect("connect should succeed");
    client.disconnect().expect("disconnect should succeed");
    let identity_after = endpoint.identity();
    assert_eq!(identity_before, identity_after);
}

#[test]
fn mpmc_queue_preserves_fifo_order_per_producer() {
    let queue = MpmcQueue::new(QueueConfig::bounded(8));
    let producer = queue.producer("p0");

    producer.enqueue(request(10)).expect("first enqueue should work");
    producer.enqueue(request(11)).expect("second enqueue should work");

    let first = queue.dequeue(Timeout::from_ticks(1)).expect("first dequeue should succeed");
    let second = queue.dequeue(Timeout::from_ticks(1)).expect("second dequeue should succeed");

    assert_eq!(first.id(), 10);
    assert_eq!(second.id(), 11);
}

#[test]
fn mpmc_queue_preserves_global_order_for_equal_priority() {
    let queue = MpmcQueue::new(QueueConfig::bounded(8));

    queue
        .enqueue_with_priority(request(20), RequestPriority::Normal)
        .expect("first enqueue should work");
    queue
        .enqueue_with_priority(request(21), RequestPriority::Normal)
        .expect("second enqueue should work");

    let first = queue.dequeue(Timeout::from_ticks(1)).expect("first dequeue should succeed");
    let second = queue.dequeue(Timeout::from_ticks(1)).expect("second dequeue should succeed");

    assert_eq!(first.id(), 20);
    assert_eq!(second.id(), 21);
}

#[test]
fn mpmc_queue_reports_backpressure_at_capacity() {
    let queue = MpmcQueue::new(QueueConfig::bounded(2));

    queue.try_enqueue(request(30)).expect("first enqueue should work");
    queue.try_enqueue(request(31)).expect("second enqueue should work");

    let saturated = queue.try_enqueue(request(32));
    assert!(matches!(saturated, Err(QueueError::Backpressure)));
}

#[test]
fn mpmc_queue_dequeue_returns_timeout_when_empty() {
    let clock = DeterministicClock::from_tick(100);
    let queue = MpmcQueue::with_clock(QueueConfig::bounded(4), clock.clone());

    let timed_out = queue.dequeue(Timeout::from_ticks(5));
    assert!(matches!(timed_out, Err(QueueError::TimeoutAtTick(105))));
}

#[test]
fn mpmc_queue_metrics_track_depth_and_rejections() {
    let queue = MpmcQueue::new(QueueConfig::bounded(1));

    queue.try_enqueue(request(40)).expect("first enqueue should work");
    let _ = queue.try_enqueue(request(41));

    let metrics = queue.metrics();
    assert_eq!(metrics.depth, 1);
    assert_eq!(metrics.rejected, 1);
}

#[test]
fn concurrent_producers_enqueue_without_lost_messages() {
    let queue = Arc::new(MpmcQueue::new(QueueConfig::bounded(256)));

    let queue_a = Arc::clone(&queue);
    let producer_a = thread::spawn(move || {
        for id in 0_u64..50 {
            queue_a.try_enqueue(request(id)).expect("producer A enqueue should work");
        }
    });

    let queue_b = Arc::clone(&queue);
    let producer_b = thread::spawn(move || {
        for id in 50_u64..100 {
            queue_b.try_enqueue(request(id)).expect("producer B enqueue should work");
        }
    });

    producer_a.join().expect("producer A should join");
    producer_b.join().expect("producer B should join");

    let mut seen = BTreeSet::new();
    for _ in 0..100 {
        let next = queue
            .dequeue(Timeout::from_ticks(1))
            .expect("dequeue should succeed for all produced messages");
        seen.insert(next.id());
    }

    assert_eq!(seen.len(), 100);
}

#[test]
fn concurrent_consumers_receive_disjoint_message_sets() {
    let queue = Arc::new(MpmcQueue::new(QueueConfig::bounded(256)));

    for id in 0_u64..100 {
        queue.try_enqueue(request(id)).expect("prefill enqueue should work");
    }

    let queue_a = Arc::clone(&queue);
    let consumer_a = thread::spawn(move || {
        let mut ids = Vec::new();
        for _ in 0..50 {
            ids.push(
                queue_a
                    .dequeue(Timeout::from_ticks(1))
                    .expect("consumer A dequeue should work")
                    .id(),
            );
        }
        ids
    });

    let queue_b = Arc::clone(&queue);
    let consumer_b = thread::spawn(move || {
        let mut ids = Vec::new();
        for _ in 0..50 {
            ids.push(
                queue_b
                    .dequeue(Timeout::from_ticks(1))
                    .expect("consumer B dequeue should work")
                    .id(),
            );
        }
        ids
    });

    let set_a: BTreeSet<u64> =
        consumer_a.join().expect("consumer A should join").into_iter().collect();
    let set_b: BTreeSet<u64> =
        consumer_b.join().expect("consumer B should join").into_iter().collect();

    assert!(set_a.is_disjoint(&set_b));
    assert_eq!(set_a.len() + set_b.len(), 100);
}

#[test]
fn cancelled_dequeue_does_not_drop_enqueued_message() {
    let queue = MpmcQueue::new(QueueConfig::bounded(8));
    queue.try_enqueue(request(777)).expect("enqueue should work");

    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let cancelled = queue.dequeue_with_cancellation(Timeout::from_ticks(100), &cancellation);
    assert!(matches!(cancelled, Err(QueueError::Cancelled)));

    let preserved = queue
        .dequeue(Timeout::from_ticks(1))
        .expect("message should remain queued after cancelled dequeue");
    assert_eq!(preserved.id(), 777);
}

#[test]
fn timeout_fires_at_exact_deadline_tick() {
    let clock = DeterministicClock::from_tick(1000);
    let queue = MpmcQueue::with_clock(QueueConfig::bounded(4), clock.clone());

    let result = queue.dequeue(Timeout::from_ticks(25));
    assert!(matches!(result, Err(QueueError::TimeoutAtTick(1025))));
    assert_eq!(clock.now_tick(), 1025);
}

#[test]
fn cancellation_short_circuits_wait_before_timeout() {
    let clock = DeterministicClock::from_tick(200);
    let queue = MpmcQueue::with_clock(QueueConfig::bounded(4), clock.clone());
    let cancellation = CancellationToken::cancel_at_tick(203);

    let result = queue.dequeue_with_cancellation(Timeout::from_ticks(50), &cancellation);
    assert!(matches!(result, Err(QueueError::CancelledAtTick(203))));
    assert_eq!(clock.now_tick(), 203);
}

#[test]
fn cancellation_wins_when_timeout_and_cancel_share_tick() {
    let clock = DeterministicClock::from_tick(300);
    let queue = MpmcQueue::with_clock(QueueConfig::bounded(4), clock.clone());
    let cancellation = CancellationToken::cancel_at_tick(310);

    let result = queue.dequeue_with_cancellation(Timeout::from_ticks(10), &cancellation);
    assert!(matches!(result, Err(QueueError::CancelledAtTick(310))));
    assert_eq!(clock.now_tick(), 310);
}
