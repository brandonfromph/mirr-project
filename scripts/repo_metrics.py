#!/usr/bin/env python3
"""Repository metrics collector for Proposal 096 closeout."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Dict, Iterable, List

ROOT = Path(__file__).resolve().parents[1]


def list_rust_files(paths: Iterable[Path]) -> List[Path]:
    files: List[Path] = []
    for path in paths:
        if path.exists():
            files.extend(path.rglob("*.rs"))
    return files


def crate_source_roots(crates_dir: Path) -> List[Path]:
    roots: List[Path] = []
    if not crates_dir.exists():
        return roots
    for entry in crates_dir.iterdir():
        if not entry.is_dir():
            continue
        src_dir = entry / "src"
        if src_dir.exists():
            roots.append(src_dir)
    return roots


def count_pattern(files: Iterable[Path], pattern: re.Pattern[str]) -> int:
    total = 0
    for file_path in files:
        try:
            content = file_path.read_text(encoding="utf-8")
        except OSError:
            continue
        total += len(pattern.findall(content))
    return total


def generate_metrics(root: Path) -> Dict[str, object]:
    src_dir = root / "src"
    tests_dir = root / "tests"
    crates_dir = root / "crates"
    proposals_dir = root / "proposals"

    scan_roots = [src_dir, tests_dir]
    scan_roots.extend(crate_source_roots(crates_dir))

    src_files = list_rust_files([src_dir])
    test_files = list_rust_files([tests_dir])
    scan_files = list_rust_files(scan_roots)

    violations = {
        "unsafe_keyword": count_pattern(scan_files, re.compile(r"\bunsafe\b")),
        "deprecated_attr": count_pattern(scan_files, re.compile(r"#\s*\[\s*deprecated")),
        "allow_dead_code": count_pattern(
            scan_files,
            re.compile(r"#\s*\[\s*allow\s*\(\s*dead_code\s*\)\s*\]"),
        ),
    }

    kb_root = root / ".kb-data"
    graph_db = kb_root / "graph.db"
    lance_root = kb_root / "knowledge.lance"
    lance_data = lance_root / "data"
    lance_txn = lance_root / "_transactions"
    lance_versions = lance_root / "_versions"

    graph_db_bytes = graph_db.stat().st_size if graph_db.exists() else 0
    lance_data_files = len(list(lance_data.glob("*"))) if lance_data.exists() else 0
    lance_txn_files = len(list(lance_txn.glob("*"))) if lance_txn.exists() else 0
    lance_version_files = len(list(lance_versions.glob("*"))) if lance_versions.exists() else 0

    proposals_count = len(list(proposals_dir.rglob("*.md"))) if proposals_dir.exists() else 0

    return {
        "src_rust_files": len(src_files),
        "tests_rust_files": len(test_files),
        "proposals_count": proposals_count,
        "unsafe_keyword_count": violations["unsafe_keyword"],
        "deprecated_attr_count": violations["deprecated_attr"],
        "allow_dead_code_count": violations["allow_dead_code"],
        "kb_data_present": kb_root.exists(),
        "graph_db_present": graph_db.exists(),
        "graph_db_bytes": graph_db_bytes,
        "lance_data_files": lance_data_files,
        "lance_txn_files": lance_txn_files,
        "lance_version_files": lance_version_files,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Collect repository metrics.")
    parser.add_argument("--json", action="store_true", help="Print metrics as JSON")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    metrics = generate_metrics(ROOT)

    if args.json:
        print(json.dumps(metrics, indent=2))
    else:
        for key, value in metrics.items():
            print(f"{key}={value}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
