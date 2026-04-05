#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

use super::cache::{should_skip_package, CacheManifest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WaveKind {
    Fmt,
    Check,
    Clippy,
    Nextest,
    Rocq,
    Parity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskSpec {
    pub wave_index: usize,
    pub package_name: String,
    pub command: OsString,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, OsString>,
    pub allow_cache_skip: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaveSpec {
    pub wave_index: usize,
    pub kind: WaveKind,
    pub tasks: Vec<TaskSpec>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlan {
    pub waves: Vec<WaveSpec>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskResult {
    pub wave_index: usize,
    pub package_name: String,
    pub status_code: i32,
    pub skipped_by_cache: bool,
    pub command_line: String,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaveResult {
    pub wave_index: usize,
    pub wave_kind: WaveKind,
    pub task_results: Vec<TaskResult>,
    pub failed: bool,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunSummary {
    pub success: bool,
    pub completed_wave: usize,
    pub task_results: Vec<TaskResult>,
    pub wave_results: Vec<WaveResult>,
    pub total_duration_ms: u64,
}

#[derive(Clone, Debug)]
struct IndexedTask {
    index: usize,
    task: TaskSpec,
}

#[derive(Clone, Debug)]
struct IndexedTaskResult {
    index: usize,
    result: TaskResult,
}

fn render_command_line(task: &TaskSpec) -> io::Result<String> {
    let mut parts = Vec::with_capacity(task.args.len() + 1);
    parts.push(task.command.to_string_lossy().to_string());
    for arg in &task.args {
        parts.push(arg.to_string_lossy().to_string());
    }
    Ok(parts.join(" "))
}

fn validate_wave_tasks(wave: &WaveSpec) -> io::Result<()> {
    let mut seen_packages: BTreeMap<&str, ()> = BTreeMap::new();

    for task in &wave.tasks {
        if seen_packages.insert(task.package_name.as_str(), ()).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "duplicate package_name '{}' in wave {}",
                    task.package_name, wave.wave_index
                ),
            ));
        }
    }

    Ok(())
}

pub(crate) fn worker_pool_size() -> io::Result<usize> {
    let size = num_cpus::get().clamp(1, 4);
    Ok(size)
}

fn run_task(task: &TaskSpec) -> io::Result<TaskResult> {
    let started = Instant::now();
    let command_line = render_command_line(task)?;

    let mut command = Command::new(&task.command);
    command.args(&task.args);
    command.current_dir(&task.cwd);
    for (key, value) in &task.env {
        command.env(key, value);
    }

    let status = command.status()?;
    let status_code = status.code().unwrap_or(1);

    Ok(TaskResult {
        wave_index: task.wave_index,
        package_name: task.package_name.clone(),
        status_code,
        skipped_by_cache: false,
        command_line,
        duration_ms: started.elapsed().as_millis() as u64,
    })
}

fn worker_loop(
    receiver: Arc<Mutex<mpsc::Receiver<IndexedTask>>>,
    result_sender: mpsc::Sender<IndexedTaskResult>,
) -> io::Result<()> {
    loop {
        let next_task = {
            let guard = receiver.lock().map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "task receiver mutex poisoned")
            })?;
            guard.recv()
        };

        let indexed_task = match next_task {
            Ok(task) => task,
            Err(_) => break,
        };

        let result = match run_task(&indexed_task.task) {
            Ok(task_result) => task_result,
            Err(_) => TaskResult {
                wave_index: indexed_task.task.wave_index,
                package_name: indexed_task.task.package_name.clone(),
                status_code: 1,
                skipped_by_cache: false,
                command_line: "IO_ERROR:".to_string(),
                duration_ms: 0,
            },
        };

        if result_sender.send(IndexedTaskResult { index: indexed_task.index, result }).is_err() {
            break;
        }
    }

    Ok(())
}

pub fn execute_wave_barrier(
    plan: &ExecutionPlan,
    wave_index: usize,
    manifest: &CacheManifest,
    fingerprints: &BTreeMap<String, String>,
) -> io::Result<WaveResult> {
    let wave_started = Instant::now();
    let wave = plan.waves.get(wave_index).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("wave index {} is out of range", wave_index),
        )
    })?;

    validate_wave_tasks(wave)?;

    let mut ordered_results: Vec<Option<TaskResult>> = vec![None; wave.tasks.len()];
    let mut to_run: Vec<IndexedTask> = Vec::new();

    for (index, task) in wave.tasks.iter().cloned().enumerate() {
        let fingerprint = fingerprints.get(&task.package_name);
        let should_skip = wave_index != 0
            && task.allow_cache_skip
            && fingerprint
                .map(|value| should_skip_package(manifest, &task.package_name, value))
                .unwrap_or(false);

        if should_skip {
            let command_line = render_command_line(&task)?;
            ordered_results[index] = Some(TaskResult {
                wave_index: task.wave_index,
                package_name: task.package_name,
                status_code: 0,
                skipped_by_cache: true,
                command_line,
                duration_ms: 0,
            });
        } else {
            to_run.push(IndexedTask { index, task });
        }
    }

    if to_run.is_empty() {
        let mut failed = false;
        let mut task_results = Vec::with_capacity(wave.tasks.len());
        for result in ordered_results.into_iter().flatten() {
            if result.status_code != 0 {
                failed = true;
            }
            task_results.push(result);
        }

        return Ok(WaveResult {
            wave_index,
            wave_kind: wave.kind.clone(),
            task_results,
            failed,
            duration_ms: wave_started.elapsed().as_millis() as u64,
        });
    }

    let worker_count = worker_pool_size()?.min(to_run.len());
    let (task_tx, task_rx) = mpsc::channel::<IndexedTask>();
    let (result_tx, result_rx) = mpsc::channel::<IndexedTaskResult>();
    let shared_rx = Arc::new(Mutex::new(task_rx));

    let mut handles = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let rx = Arc::clone(&shared_rx);
        let tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            let _ = worker_loop(rx, tx);
        }));
    }
    drop(result_tx);

    for indexed_task in to_run {
        task_tx.send(indexed_task).map_err(|_| {
            io::Error::new(io::ErrorKind::BrokenPipe, "task dispatcher channel closed")
        })?;
    }
    drop(task_tx);

    let mut received = 0usize;
    let to_collect = ordered_results.iter().filter(|entry| entry.is_none()).count();
    while received < to_collect {
        let indexed_result = result_rx.recv().map_err(|_| {
            io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "worker pool ended before all task results were collected",
            )
        })?;
        ordered_results[indexed_result.index] = Some(indexed_result.result);
        received += 1;
    }

    for handle in handles {
        let _ = handle.join();
    }

    let mut failed = false;
    let mut task_results = Vec::with_capacity(wave.tasks.len());
    for result in ordered_results.into_iter().flatten() {
        if result.status_code != 0 {
            failed = true;
        }
        task_results.push(result);
    }

    Ok(WaveResult {
        wave_index,
        wave_kind: wave.kind.clone(),
        task_results,
        failed,
        duration_ms: wave_started.elapsed().as_millis() as u64,
    })
}

pub fn execute_all_waves(
    plan: &ExecutionPlan,
    manifest: &CacheManifest,
    fingerprints: &BTreeMap<String, String>,
) -> io::Result<RunSummary> {
    let started = Instant::now();
    let mut task_results = Vec::new();
    let mut wave_results = Vec::new();
    let mut completed_wave = 0usize;
    let mut success = true;

    for wave_index in 0..plan.waves.len() {
        let wave_result = execute_wave_barrier(plan, wave_index, manifest, fingerprints)?;
        completed_wave += 1;
        task_results.extend(wave_result.task_results.clone());
        let wave_failed = wave_result.failed;
        wave_results.push(wave_result);

        if wave_failed {
            success = false;
            break;
        }
    }

    Ok(RunSummary {
        success,
        completed_wave,
        task_results,
        wave_results,
        total_duration_ms: started.elapsed().as_millis() as u64,
    })
}
