#!/usr/bin/env python3
"""
MIRR Research Experiment Runner

Generates a reproducible evidence pack from the current MIRR compiler
implementation (no FPGA required):

1) Temporal strategy sweep (ShiftRegister vs Counter)
2) Determinism runs (hash stability)
3) Throughput baseline (median/p95/stddev)
4) Bootstrap failure-mode checks (read/parse/validate pipeline behavior)

Outputs CSV + Markdown summary artifacts under --out-dir.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import re
import statistics
import subprocess
import sys
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SHIFT_REGISTER_THRESHOLD = 16
DEFAULT_DELAYS = [0, 1, 2, 4, 8, 16, 17, 32, 64, 128, 256, 512, 1000, 2000]
DEFAULT_FIXTURES = ["examples/neonatal_respirator.mirr"]

def parse_delays(raw: str) -> list[int]:
    values: list[int] = []
    for token in raw.split(","):
        token = token.strip()
        if not token:
            continue
        value = int(token)
        if value < 0:
            raise ValueError(f"Delay must be non-negative, got {value}")
        values.append(value)
    if not values:
        raise ValueError("At least one delay must be provided")
    return sorted(set(values))

def run_command(
    cmd: list[str],
    cwd: Path,
    *,
    allow_failure: bool = False,
) -> subprocess.CompletedProcess[str]:
    proc = subprocess.run(
        cmd,
        cwd=str(cwd),
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0 and not allow_failure:
        raise RuntimeError(
            "Command failed:\n"
            f"  {' '.join(cmd)}\n"
            f"  exit={proc.returncode}\n"
            f"  stdout:\n{proc.stdout}\n"
            f"  stderr:\n{proc.stderr}"
        )
    return proc

def percentile(values: list[float], q: float) -> float:
    if not values:
        return 0.0
    if len(values) == 1:
        return values[0]
    sorted_vals = sorted(values)
    idx = (len(sorted_vals) - 1) * q
    low = int(idx)
    high = min(low + 1, len(sorted_vals) - 1)
    frac = idx - low
    return sorted_vals[low] * (1.0 - frac) + sorted_vals[high] * frac

def ensure_release_binary(project_root: Path, skip_build: bool) -> Path:
    exe_name = "nasa-rust-project.exe" if os.name == "nt" else "nasa-rust-project"
    binary = project_root / "target" / "release" / exe_name

    if not skip_build:
        print("[build] cargo build --release")
        run_command(["cargo", "build", "--release"], project_root)

    if not binary.exists():
        raise FileNotFoundError(
            f"Release binary not found at {binary}. Run cargo build --release first."
        )
    return binary

def compile_to_json(binary: Path, project_root: Path, input_file: Path) -> tuple[dict[str, Any], str]:
    proc = run_command(
        [str(binary), "--compile", "--json", str(input_file)],
        project_root,
    )
    raw = proc.stdout.strip()
    try:
        parsed = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise RuntimeError(
            f"Failed to parse JSON output for {input_file}: {exc}\nOutput:\n{raw}"
        ) from exc
    return parsed, raw

def run_selfhost(binary: Path, project_root: Path, input_file: Path) -> dict[str, Any]:
    proc = run_command(
        [str(binary), "--selfhost-compile", str(input_file)],
        project_root,
        allow_failure=True,
    )

    stage_pattern = re.compile(r"Stage\s+\d+:\s+(?P<icon>[✓✗])\s+\[(?P<name>[^\]]+)\]\s+(?P<msg>.*)")
    summary_pattern = re.compile(r"\[SELF-HOST\s+(?P<status>PASS|FAIL)\].*")

    stages: list[dict[str, str]] = []
    for line in proc.stdout.splitlines():
        m = stage_pattern.search(line)
        if m:
            stages.append(
                {
                    "icon": m.group("icon"),
                    "name": m.group("name"),
                    "message": m.group("msg"),
                }
            )

    summary_status = None
    summary_line = ""
    merged_stream = proc.stdout + "\n" + proc.stderr
    for line in merged_stream.splitlines():
        m = summary_pattern.search(line)
        if m:
            summary_status = m.group("status")
            summary_line = line.strip()
            break

    first_failed_stage = ""
    for st in stages:
        if st["icon"] == "✗":
            first_failed_stage = st["name"]
            break

    observed_success = summary_status == "PASS" or proc.returncode == 0

    return {
        "return_code": proc.returncode,
        "observed_success": observed_success,
        "summary_status": summary_status or ("PASS" if observed_success else "FAIL"),
        "summary_line": summary_line,
        "first_failed_stage": first_failed_stage,
        "stage_count": len(stages),
    }

def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        path.write_text("", encoding="utf-8")
        return
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

def strategy_sweep(
    binary: Path,
    project_root: Path,
    out_tmp: Path,
    delays: list[int],
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    rows: list[dict[str, Any]] = []

    for delay in delays:
        module = f"""
module sweep_delay_{delay} {{
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard delay_guard_{delay} {{
        when input_signal
        for {delay} cycles;
    }}

    reflex r_{delay} {{
        on delay_guard_{delay} {{
            output_signal = input_signal;
        }}
    }}
}}
""".strip()
        mirr_path = out_tmp / f"delay_{delay}.mirr"
        mirr_path.write_text(module + "\n", encoding="utf-8")

        netlist, _raw = compile_to_json(binary, project_root, mirr_path)
        guard_obj = netlist["guards"][0]
        chosen_strategy = (
            "ShiftRegister" if "ShiftRegister" in guard_obj else "Counter" if "Counter" in guard_obj else "Unknown"
        )
        expected_strategy = "ShiftRegister" if delay <= SHIFT_REGISTER_THRESHOLD else "Counter"
        matched = chosen_strategy == expected_strategy

        stats = netlist.get("statistics", {})
        rows.append(
            {
                "delay_cycles": delay,
                "expected_strategy": expected_strategy,
                "chosen_strategy": chosen_strategy,
                "matches_expected": matched,
                "shift_registers_used": stats.get("shift_registers_used"),
                "counters_used": stats.get("counters_used"),
                "logic_gates_used": stats.get("logic_gates_used"),
                "total_signals": stats.get("total_signals"),
            }
        )

    total = len(rows)
    matched_count = sum(1 for r in rows if r["matches_expected"])
    accuracy = (matched_count / total) if total else 0.0

    summary = {
        "total_cases": total,
        "matched_cases": matched_count,
        "accuracy": accuracy,
        "threshold_cycles": SHIFT_REGISTER_THRESHOLD,
    }
    return rows, summary

def determinism_experiment(
    binary: Path,
    project_root: Path,
    fixture: Path,
    runs: int,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    digests: list[str] = []

    for i in range(1, runs + 1):
        _parsed, raw = compile_to_json(binary, project_root, fixture)
        digest = hashlib.sha256(raw.encode("utf-8")).hexdigest()
        digests.append(digest)
        rows.append(
            {
                "run": i,
                "sha256": digest,
                "bytes": len(raw.encode("utf-8")),
            }
        )

    counter = Counter(digests)
    dominant_digest, dominant_count = counter.most_common(1)[0]
    unique_hashes = len(counter)

    summary = {
        "runs": runs,
        "unique_hashes": unique_hashes,
        "dominant_hash": dominant_digest,
        "dominant_count": dominant_count,
        "mismatch_count": runs - dominant_count,
        "deterministic": unique_hashes == 1,
    }
    return rows, summary

def throughput_experiment(
    binary: Path,
    project_root: Path,
    fixtures: list[Path],
    warmup_runs: int,
    benchmark_runs: int,
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []

    for fixture in fixtures:
        for _ in range(warmup_runs):
            compile_to_json(binary, project_root, fixture)

        timings_ms: list[float] = []
        for _ in range(benchmark_runs):
            t0 = time.perf_counter()
            compile_to_json(binary, project_root, fixture)
            dt = (time.perf_counter() - t0) * 1000.0
            timings_ms.append(dt)

        row = {
            "fixture": str(fixture.relative_to(project_root)),
            "runs": benchmark_runs,
            "mean_ms": round(statistics.fmean(timings_ms), 4),
            "median_ms": round(statistics.median(timings_ms), 4),
            "p95_ms": round(percentile(timings_ms, 0.95), 4),
            "stddev_ms": round(statistics.stdev(timings_ms), 4) if len(timings_ms) > 1 else 0.0,
            "min_ms": round(min(timings_ms), 4),
            "max_ms": round(max(timings_ms), 4),
        }
        rows.append(row)

    return rows

def bootstrap_failure_modes(binary: Path, project_root: Path, out_tmp: Path) -> list[dict[str, Any]]:
    cases: list[dict[str, Any]] = []

    # Case 1: canonical expected PASS
    canonical = project_root / "examples" / "neonatal_respirator.mirr"
    result_ok = run_selfhost(binary, project_root, canonical)
    cases.append(
        {
            "case": "canonical_example",
            "expected_success": True,
            "observed_success": result_ok["observed_success"],
            "return_code": result_ok["return_code"],
            "summary_status": result_ok["summary_status"],
            "first_failed_stage": result_ok["first_failed_stage"],
            "summary_line": result_ok["summary_line"],
        }
    )

    # Case 2: parse failure expected
    malformed = out_tmp / "malformed_input.mirr"
    malformed.write_text("module broken_syntax {\n signal x: in bool\n", encoding="utf-8")
    result_parse_fail = run_selfhost(binary, project_root, malformed)
    cases.append(
        {
            "case": "malformed_parse_error",
            "expected_success": False,
            "observed_success": result_parse_fail["observed_success"],
            "return_code": result_parse_fail["return_code"],
            "summary_status": result_parse_fail["summary_status"],
            "first_failed_stage": result_parse_fail["first_failed_stage"],
            "summary_line": result_parse_fail["summary_line"],
        }
    )

    # Case 3: read failure expected
    missing = out_tmp / "missing_file_should_fail.mirr"
    result_read_fail = run_selfhost(binary, project_root, missing)
    cases.append(
        {
            "case": "missing_file_read_error",
            "expected_success": False,
            "observed_success": result_read_fail["observed_success"],
            "return_code": result_read_fail["return_code"],
            "summary_status": result_read_fail["summary_status"],
            "first_failed_stage": result_read_fail["first_failed_stage"],
            "summary_line": result_read_fail["summary_line"],
        }
    )

    return cases

def git_head(project_root: Path) -> str:
    proc = run_command(["git", "rev-parse", "--short", "HEAD"], project_root, allow_failure=True)
    if proc.returncode == 0:
        return proc.stdout.strip()
    return "unknown"

def write_markdown_summary(
    out_path: Path,
    metadata: dict[str, Any],
    strategy_summary: dict[str, Any],
    determinism_summary: dict[str, Any],
    throughput_rows: list[dict[str, Any]],
    bootstrap_rows: list[dict[str, Any]],
) -> None:
    lines: list[str] = []
    lines.append("# MIRR Tangible Evidence Summary")
    lines.append("")
    lines.append(f"- UTC timestamp: `{metadata['timestamp_utc']}`")
    lines.append(f"- Commit: `{metadata['git_commit']}`")
    lines.append(f"- Release binary: `{metadata['binary']}`")
    lines.append("")

    lines.append("## 1) Temporal strategy sweep")
    lines.append("")
    lines.append(
        "- Threshold tested: "
        f"`N <= {strategy_summary['threshold_cycles']} => ShiftRegister`, "
        f"`N > {strategy_summary['threshold_cycles']} => Counter`"
    )
    lines.append(
        f"- Cases matched expectation: `{strategy_summary['matched_cases']}/{strategy_summary['total_cases']}`"
    )
    lines.append(f"- Accuracy: `{strategy_summary['accuracy']:.2%}`")
    lines.append("")

    lines.append("## 2) Determinism")
    lines.append("")
    lines.append(f"- Runs: `{determinism_summary['runs']}`")
    lines.append(f"- Unique output hashes: `{determinism_summary['unique_hashes']}`")
    lines.append(f"- Deterministic: `{determinism_summary['deterministic']}`")
    lines.append(f"- Mismatch count: `{determinism_summary['mismatch_count']}`")
    lines.append("")

    lines.append("## 3) Throughput baseline (compile --json)")
    lines.append("")
    lines.append("| Fixture | Mean (ms) | Median (ms) | p95 (ms) | Stddev (ms) |")
    lines.append("|---|---:|---:|---:|---:|")
    for row in throughput_rows:
        lines.append(
            f"| `{row['fixture']}` | {row['mean_ms']} | {row['median_ms']} | {row['p95_ms']} | {row['stddev_ms']} |"
        )
    lines.append("")

    lines.append("## 4) Bootstrap failure modes")
    lines.append("")
    lines.append("| Case | Expected Success | Observed Success | First Failed Stage |")
    lines.append("|---|---:|---:|---|")
    for row in bootstrap_rows:
        first_failed = row["first_failed_stage"] or "(none)"
        lines.append(
            f"| `{row['case']}` | {row['expected_success']} | {row['observed_success']} | {first_failed} |"
        )
    lines.append("")

    lines.append("## Artifact files")
    lines.append("")
    lines.append("- `strategy_sweep.csv`")
    lines.append("- `determinism_runs.csv`")
    lines.append("- `throughput_baseline.csv`")
    lines.append("- `bootstrap_failure_modes.csv`")
    lines.append("- `run_metadata.json`")
    lines.append("")

    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

def main() -> int:
    parser = argparse.ArgumentParser(description="Run MIRR research evidence experiments")
    parser.add_argument("--project-root", default=".", help="Repository root (default: .)")
    parser.add_argument(
        "--out-dir",
        default="artifacts/research",
        help="Output directory for CSV/Markdown artifacts",
    )
    parser.add_argument(
        "--delays",
        default=",".join(str(d) for d in DEFAULT_DELAYS),
        help="Comma-separated delay values for strategy sweep",
    )
    parser.add_argument("--determinism-runs", type=int, default=20)
    parser.add_argument("--warmup-runs", type=int, default=3)
    parser.add_argument("--benchmark-runs", type=int, default=10)
    parser.add_argument(
        "--fixtures",
        nargs="*",
        default=DEFAULT_FIXTURES,
        help="Fixture paths for throughput experiment",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Skip cargo build --release (uses existing binary)",
    )
    args = parser.parse_args()

    project_root = Path(args.project_root).resolve()
    out_dir = (project_root / args.out_dir).resolve()
    out_tmp = out_dir / "tmp_inputs"
    out_tmp.mkdir(parents=True, exist_ok=True)

    delays = parse_delays(args.delays)
    if args.determinism_runs < 1:
        raise ValueError("--determinism-runs must be >= 1")
    if args.warmup_runs < 0:
        raise ValueError("--warmup-runs must be >= 0")
    if args.benchmark_runs < 1:
        raise ValueError("--benchmark-runs must be >= 1")

    fixtures = [
        (project_root / f).resolve()
        for f in args.fixtures
        if (project_root / f).exists()
    ]
    if not fixtures:
        raise FileNotFoundError("No valid fixture paths were found for throughput experiment")

    canonical = (project_root / "examples" / "neonatal_respirator.mirr").resolve()
    if not canonical.exists():
        raise FileNotFoundError(f"Canonical fixture missing: {canonical}")

    binary = ensure_release_binary(project_root, args.skip_build)

    print("[exp] strategy sweep")
    strategy_rows, strategy_summary = strategy_sweep(binary, project_root, out_tmp, delays)
    write_csv(out_dir / "strategy_sweep.csv", strategy_rows)

    print("[exp] determinism")
    det_rows, det_summary = determinism_experiment(
        binary,
        project_root,
        canonical,
        args.determinism_runs,
    )
    write_csv(out_dir / "determinism_runs.csv", det_rows)

    print("[exp] throughput")
    throughput_rows = throughput_experiment(
        binary,
        project_root,
        fixtures,
        args.warmup_runs,
        args.benchmark_runs,
    )
    write_csv(out_dir / "throughput_baseline.csv", throughput_rows)

    print("[exp] bootstrap failure modes")
    bootstrap_rows = bootstrap_failure_modes(binary, project_root, out_tmp)
    write_csv(out_dir / "bootstrap_failure_modes.csv", bootstrap_rows)

    metadata = {
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "git_commit": git_head(project_root),
        "binary": str(binary),
        "strategy_threshold_cycles": SHIFT_REGISTER_THRESHOLD,
        "delays": delays,
        "determinism_runs": args.determinism_runs,
        "warmup_runs": args.warmup_runs,
        "benchmark_runs": args.benchmark_runs,
        "fixtures": [str(p.relative_to(project_root)) for p in fixtures],
        "strategy_summary": strategy_summary,
        "determinism_summary": det_summary,
    }
    (out_dir / "run_metadata.json").write_text(
        json.dumps(metadata, indent=2),
        encoding="utf-8",
    )

    write_markdown_summary(
        out_dir / "summary.md",
        metadata,
        strategy_summary,
        det_summary,
        throughput_rows,
        bootstrap_rows,
    )

    print(f"\n[done] Research artifacts written to: {out_dir}")
    print("       - strategy_sweep.csv")
    print("       - determinism_runs.csv")
    print("       - throughput_baseline.csv")
    print("       - bootstrap_failure_modes.csv")
    print("       - run_metadata.json")
    print("       - summary.md")

    return 0

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:  # pragma: no cover
        print(f"[error] {exc}", file=sys.stderr)
        raise SystemExit(1)