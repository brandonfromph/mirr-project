#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

#[path = "../src/bin/mirr_general/cache.rs"]
mod cache;
#[path = "../src/bin/mirr_general/scheduler.rs"]
mod scheduler;

use scheduler::{
    execute_all_waves, execute_wave_barrier, ExecutionPlan, TaskSpec, WaveKind, WaveSpec,
};

fn marker_command(marker_name: &str, exit_code: i32) -> (OsString, Vec<OsString>) {
    #[cfg(windows)]
    {
        (
            OsString::from("cmd"),
            vec![
                OsString::from("/C"),
                OsString::from(format!("type nul > {} & exit /B {}", marker_name, exit_code)),
            ],
        )
    }

    #[cfg(not(windows))]
    {
        (
            OsString::from("sh"),
            vec![
                OsString::from("-c"),
                OsString::from(format!("touch {}; exit {}", marker_name, exit_code)),
            ],
        )
    }
}

fn make_task(
    wave_index: usize,
    package_name: &str,
    cwd: &Path,
    marker_name: &str,
    exit_code: i32,
) -> TaskSpec {
    let (command, args) = marker_command(marker_name, exit_code);
    TaskSpec {
        wave_index,
        package_name: package_name.to_string(),
        command,
        args,
        cwd: cwd.to_path_buf(),
        env: BTreeMap::new(),
        allow_cache_skip: false,
    }
}

fn make_wave(wave_index: usize, tasks: Vec<TaskSpec>) -> WaveSpec {
    WaveSpec { wave_index, kind: WaveKind::Nextest, tasks }
}

fn marker_path(dir: &Path, name: &str) -> PathBuf {
    dir.join(name)
}

fn empty_manifest() -> cache::CacheManifest {
    cache::CacheManifest { entries: BTreeMap::new() }
}

#[test]
fn wave_barrier_stops_on_first_failed_wave_does_not_execute_wave_2() -> io::Result<()> {
    let temp = tempdir()?;
    let wave0_marker = marker_path(temp.path(), "wave0.txt");
    let wave1_marker = marker_path(temp.path(), "wave1.txt");

    let plan = ExecutionPlan {
        waves: vec![
            make_wave(0, vec![make_task(0, "pkg0", temp.path(), "wave0.txt", 1)]),
            make_wave(1, vec![make_task(1, "pkg1", temp.path(), "wave1.txt", 0)]),
        ],
    };

    let fingerprints = BTreeMap::new();
    let summary = execute_all_waves(&plan, &empty_manifest(), &fingerprints)?;

    assert!(!summary.success, "failing wave should make summary unsuccessful");
    assert_eq!(summary.completed_wave, 1, "only wave 0 should complete");
    assert!(wave0_marker.exists(), "wave 0 task should have executed");
    assert!(!wave1_marker.exists(), "wave 2 must not execute after failure");
    Ok(())
}

#[test]
fn duplicate_package_name_in_same_wave_returns_err_before_dispatch() -> io::Result<()> {
    let temp = tempdir()?;
    let first_marker = marker_path(temp.path(), "dup0.txt");
    let second_marker = marker_path(temp.path(), "dup1.txt");

    let plan = ExecutionPlan {
        waves: vec![make_wave(
            0,
            vec![
                make_task(0, "pkg", temp.path(), "dup0.txt", 0),
                make_task(0, "pkg", temp.path(), "dup1.txt", 0),
            ],
        )],
    };

    let fingerprints = BTreeMap::new();
    let result = execute_wave_barrier(&plan, 0, &empty_manifest(), &fingerprints);

    assert!(result.is_err(), "duplicate package names must fail before dispatch");
    assert!(!first_marker.exists(), "first task must not run after duplicate detection");
    assert!(!second_marker.exists(), "second task must not run after duplicate detection");
    Ok(())
}

#[test]
fn successful_two_wave_plan_completes_and_summary_success_is_true() -> io::Result<()> {
    let temp = tempdir()?;
    let wave0_marker = marker_path(temp.path(), "success0.txt");
    let wave1_marker = marker_path(temp.path(), "success1.txt");

    let plan = ExecutionPlan {
        waves: vec![
            make_wave(0, vec![make_task(0, "pkg0", temp.path(), "success0.txt", 0)]),
            make_wave(1, vec![make_task(1, "pkg1", temp.path(), "success1.txt", 0)]),
        ],
    };

    let fingerprints = BTreeMap::new();
    let summary = execute_all_waves(&plan, &empty_manifest(), &fingerprints)?;

    assert!(summary.success, "all waves should succeed");
    assert_eq!(summary.completed_wave, 2, "both waves should complete");
    assert!(wave0_marker.exists(), "wave 0 task should have executed");
    assert!(wave1_marker.exists(), "wave 1 task should have executed");
    Ok(())
}

#[test]
fn fail_fast_wave_0_fails_task_results_has_exactly_1_entry() -> io::Result<()> {
    let temp = tempdir()?;
    let wave0_marker = marker_path(temp.path(), "fail0.txt");
    let wave1_marker = marker_path(temp.path(), "fail1.txt");

    let plan = ExecutionPlan {
        waves: vec![
            make_wave(0, vec![make_task(0, "pkg0", temp.path(), "fail0.txt", 1)]),
            make_wave(1, vec![make_task(1, "pkg1", temp.path(), "fail1.txt", 0)]),
        ],
    };

    let fingerprints = BTreeMap::new();
    let summary = execute_all_waves(&plan, &empty_manifest(), &fingerprints)?;

    assert!(!summary.success, "first wave failure should fail the run");
    assert_eq!(summary.task_results.len(), 1, "only the first wave should contribute results");
    assert!(wave0_marker.exists(), "failed task still runs and leaves its marker");
    assert!(!wave1_marker.exists(), "wave 1 must not run after fail-fast");
    Ok(())
}

#[test]
fn wave_with_two_independent_packages_both_complete_in_one_wave() -> io::Result<()> {
    let temp = tempdir()?;
    let alpha_marker = marker_path(temp.path(), "alpha.txt");
    let beta_marker = marker_path(temp.path(), "beta.txt");

    let plan = ExecutionPlan {
        waves: vec![make_wave(
            0,
            vec![
                make_task(0, "alpha", temp.path(), "alpha.txt", 0),
                make_task(0, "beta", temp.path(), "beta.txt", 0),
            ],
        )],
    };

    let fingerprints = BTreeMap::new();
    let wave_result = execute_wave_barrier(&plan, 0, &empty_manifest(), &fingerprints)?;

    assert_eq!(wave_result.task_results.len(), 2, "both tasks should complete in one wave");
    assert!(!wave_result.failed, "independent successful tasks must not fail the wave");
    assert!(wave_result.task_results.iter().all(|result| result.status_code == 0));
    assert!(alpha_marker.exists(), "alpha task should have executed");
    assert!(beta_marker.exists(), "beta task should have executed");
    Ok(())
}

#[test]
fn worker_pool_count_is_min_num_cpus_4_assert_count_ge_1_and_le_4() -> io::Result<()> {
    let wave_kinds = [
        WaveKind::Fmt,
        WaveKind::Check,
        WaveKind::Clippy,
        WaveKind::Nextest,
        WaveKind::Rocq,
        WaveKind::Parity,
    ];
    assert_eq!(wave_kinds.len(), 6);

    let worker_count = scheduler::worker_pool_size()?;

    assert!(worker_count >= 1, "worker count must be at least 1");
    assert!(worker_count <= 4, "worker count must never exceed 4");
    assert_eq!(worker_count, num_cpus::get().clamp(1, 4));
    Ok(())
}

#[test]
fn cached_package_is_skipped_and_marked_in_result() -> io::Result<()> {
    let temp = tempdir()?;
    let source_path = temp.path().join("pkg_cached.rs");
    std::fs::write(&source_path, "cached content")?;

    let pkg =
        cache::PackageSpec { name: "cached_pkg".to_string(), source_files: vec![source_path] };
    let fingerprint = cache::compute_package_fingerprint(&pkg, "lockhash", "rustc 1.80.0")?;

    let mut manifest = cache::CacheManifest::default();
    cache::upsert_manifest_entry(&mut manifest, "cached_pkg", &fingerprint);
    let manifest_path = temp.path().join("cache.manifest");
    cache::write_manifest_atomic(&manifest_path, &manifest)?;
    let manifest = cache::load_manifest(&manifest_path)?;

    let plan = ExecutionPlan {
        waves: vec![
            make_wave(0, vec![make_task(0, "prelude", temp.path(), "prelude.txt", 0)]),
            make_wave(
                1,
                vec![TaskSpec {
                    wave_index: 1,
                    package_name: "cached_pkg".to_string(),
                    command: OsString::from("sh"),
                    args: vec![OsString::from("-c"), OsString::from("touch should_not_exist.txt")],
                    cwd: temp.path().to_path_buf(),
                    env: BTreeMap::new(),
                    allow_cache_skip: true,
                }],
            ),
        ],
    };

    let mut fingerprints = BTreeMap::new();
    fingerprints.insert("cached_pkg".to_string(), fingerprint);

    let result = execute_wave_barrier(&plan, 1, &manifest, &fingerprints)?;

    assert_eq!(result.task_results.len(), 1);
    assert!(result.task_results[0].skipped_by_cache);
    assert_eq!(result.task_results[0].status_code, 0);
    assert!(!temp.path().join("should_not_exist.txt").exists());
    Ok(())
}
