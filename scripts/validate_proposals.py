#!/usr/bin/env python3
"""Proposal validation utility for Proposal 096 closeout gates."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Iterable, List

ROOT = Path(__file__).resolve().parents[1]
PROPOSALS_DIR = ROOT / "proposals"
KB_DIR = ROOT / ".kb-data"


def default_files() -> List[Path]:
    if not PROPOSALS_DIR.exists():
        return []
    return sorted(PROPOSALS_DIR.glob("*.md"))


def resolve_files(raw_files: Iterable[str]) -> List[Path]:
    resolved: List[Path] = []
    for item in raw_files:
        path = Path(item)
        if not path.is_absolute():
            path = ROOT / path
        resolved.append(path)
    return resolved


def validate_content(path: Path, strict: bool) -> List[str]:
    issues: List[str] = []
    text = path.read_text(encoding="utf-8")

    required_headings = ["#", "## Execution Plan", "## File Manifest"]
    for heading in required_headings:
        if heading not in text:
            issues.append(f"missing heading marker: {heading}")

    if strict and "\t" in text:
        issues.append("contains tab characters")

    return issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate MIRR proposal files.")
    parser.add_argument("--strict", action="store_true", help="Treat warnings as errors")
    parser.add_argument(
        "--kb-lite-strict",
        action="store_true",
        help="Require .kb-data/graph.db and .kb-data/knowledge.lance to exist",
    )
    parser.add_argument(
        "--files",
        nargs="*",
        default=[],
        help="Specific proposal files to validate (workspace-relative or absolute)",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    files = resolve_files(args.files) if args.files else default_files()
    warnings: List[str] = []
    errors: List[str] = []

    if not files:
        errors.append("no proposal files found to validate")

    for path in files:
        if not path.exists():
            errors.append(f"missing file: {path.relative_to(ROOT) if path.is_relative_to(ROOT) else path}")
            continue
        if path.suffix.lower() != ".md":
            warnings.append(f"non-markdown file in proposal list: {path}")
            if args.strict:
                errors.append(f"non-markdown file in strict mode: {path}")
            continue

        issues = validate_content(path, args.strict)
        if issues:
            formatted = [f"{path.relative_to(ROOT)}: {issue}" for issue in issues]
            warnings.extend(formatted)
            if args.strict:
                errors.extend(formatted)

    if args.kb_lite_strict:
        graph_db = KB_DIR / "graph.db"
        knowledge_lance = KB_DIR / "knowledge.lance"
        kb_issues: List[str] = []
        if not graph_db.exists():
            kb_issues.append("missing .kb-data/graph.db")
        if not knowledge_lance.exists():
            kb_issues.append("missing .kb-data/knowledge.lance")
        if kb_issues:
            issue_block = [f"KB-lite prerequisite validation: {len(kb_issues)} issue(s)"]
            issue_block.extend([f"  - {issue}" for issue in kb_issues])
            warnings.extend(issue_block)
            if args.strict:
                errors.extend(issue_block)

    if warnings:
        print("Proposal validation warnings:\n")
        print("\n".join(warnings))

    if errors:
        print("\nProposal validation failed with errors (strict mode):\n")
        print("\n".join(errors))
        return 1

    print(f"All {len(files)} proposal files are valid or warnings-only (strict={args.strict}).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
