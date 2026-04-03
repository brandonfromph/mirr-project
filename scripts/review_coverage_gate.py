#!/usr/bin/env python3
import argparse
import json
import os
import re
from collections import Counter, defaultdict
from pathlib import Path

HIGH_RISK = {"src", "crates", "mcp_server", "tests", "proposals"}
MEDIUM_RISK = {"docs", "scripts", "fuzz", "proofs", "demos", "vscode-mirr"}
ALL_DIRS = [
    "src",
    "crates",
    "docs",
    "scripts",
    "tests",
    "mcp_server",
    "vscode-mirr",
    "demos",
    "proofs",
    "fuzz",
    "proposals",
]

EXCLUDED_PARTS = {
    "node_modules",
    "target",
    ".git",
    ".kb-data",
    "dist",
    "build",
    "out",
    "coverage",
    ".next",
    "vendor",
}

LINK_RE = re.compile(r"\]\(([^)]+)\)")
PATHISH_RE = re.compile(r"(?:^|[\s`\"'])((?:src|crates|docs|scripts|tests|mcp_server|vscode-mirr|demos|proofs|fuzz|proposals)/[^\s`\"')]+)")

def norm_link_target(target: str) -> str:
    target = target.strip()
    target = target.split("#", 1)[0]
    target = target.split("?", 1)[0]
    target = target.strip()
    return target.replace('\\\\', '/').lstrip('./')

def is_reviewable_file(p: Path) -> bool:
    if not p.is_file():
        return False
    if any(part in EXCLUDED_PARTS for part in p.parts):
        return False
    # keep lightweight, text-like files
    banned_ext = {".png", ".jpg", ".jpeg", ".gif", ".webp", ".pdf", ".wasm", ".dll", ".exe", ".bin", ".lock"}
    if p.suffix.lower() in banned_ext:
        return False
    return True

def collect_repo_files(repo: Path):
    per_dir = defaultdict(set)
    for top in ALL_DIRS:
        root = repo / top
        if not root.exists():
            continue
        for f in root.rglob('*'):
            if is_reviewable_file(f):
                rel = f.relative_to(repo).as_posix()
                per_dir[top].add(rel)
    return per_dir

def extract_referenced_files(report_text: str):
    refs = []
    for m in LINK_RE.finditer(report_text):
        t = norm_link_target(m.group(1))
        if t:
            refs.append(t)
    for m in PATHISH_RE.finditer(report_text):
        t = norm_link_target(m.group(1))
        if t:
            refs.append(t)
    return refs

def main():
    ap = argparse.ArgumentParser(description="Coverage gate for multi-agent repository reviews")
    ap.add_argument("--repo", default=".")
    ap.add_argument("--reports", nargs='+', required=True, help="Report text files")
    ap.add_argument("--json-out", default="proposals/evidence/096/review-coverage-ledger.json")
    ap.add_argument("--md-out", default="proposals/evidence/096/review-coverage-ledger.md")
    args = ap.parse_args()

    repo = Path(args.repo).resolve()
    per_dir = collect_repo_files(repo)

    raw_refs = []
    for rpt in args.reports:
        p = Path(rpt)
        if not p.exists():
            continue
        raw_refs.extend(extract_referenced_files(p.read_text(encoding='utf-8', errors='ignore')))

    freq = Counter(raw_refs)
    referenced_existing = set()
    for r in freq:
        rp = repo / r
        if rp.exists() and rp.is_file():
            referenced_existing.add(r)

    coverage = {}
    hard_fail = False
    unread_high_risk = []

    for d in ALL_DIRS:
        total = len(per_dir.get(d, set()))
        read = len([f for f in per_dir.get(d, set()) if f in referenced_existing])
        pct = 0.0 if total == 0 else (read * 100.0 / total)
        if d in HIGH_RISK:
            threshold = 70.0
        elif d in MEDIUM_RISK:
            threshold = 40.0
        else:
            threshold = 0.0
        pass_dir = pct >= threshold if total > 0 else True
        if not pass_dir:
            hard_fail = True
        coverage[d] = {
            "total_files": total,
            "read_files": read,
            "coverage_pct": round(pct, 2),
            "threshold_pct": threshold,
            "pass": pass_dir,
        }
        if d in HIGH_RISK:
            unread = sorted([f for f in per_dir.get(d, set()) if f not in referenced_existing])
            unread_high_risk.extend(unread)

    sampled_top_200 = [p for p, _ in freq.most_common(200)]

    out_json = {
        "verdict": "PASS" if not hard_fail else "FAIL",
        "hard_fail": hard_fail,
        "totals": {
            "scoped_total_files": sum(len(v) for v in per_dir.values()),
            "scoped_read_files": len(referenced_existing),
            "scoped_coverage_pct": round(0.0 if sum(len(v) for v in per_dir.values()) == 0 else (len(referenced_existing) * 100.0 / sum(len(v) for v in per_dir.values())), 2),
        },
        "coverage_by_directory": coverage,
        "sampled_files_top_200": sampled_top_200,
        "unread_high_risk_files": sorted(unread_high_risk),
        "reports": args.reports,
    }

    json_path = repo / args.json_out
    json_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(json.dumps(out_json, indent=2), encoding='utf-8')

    lines = []
    lines.append("# Review Coverage Ledger")
    lines.append("")
    lines.append(f"Verdict: **{out_json['verdict']}**")
    lines.append("")
    lines.append("| Directory | Total Files | Read Files | Coverage % | Threshold % | Pass |")
    lines.append("|---|---:|---:|---:|---:|---|")
    for d in ALL_DIRS:
        c = coverage[d]
        lines.append(f"| {d} | {c['total_files']} | {c['read_files']} | {c['coverage_pct']} | {c['threshold_pct']} | {str(c['pass']).upper()} |")

    lines.append("")
    lines.append("## Sampled Files (Top 200 by Reference Frequency)")
    for p in sampled_top_200:
        lines.append(f"- {p}")

    lines.append("")
    lines.append("## Unread High-Risk Files")
    for p in sorted(unread_high_risk):
        lines.append(f"- {p}")

    md_path = repo / args.md_out
    md_path.write_text("\n".join(lines) + "\n", encoding='utf-8')

    print(json.dumps({"verdict": out_json["verdict"], "json": str(json_path), "md": str(md_path)}))
    raise SystemExit(1 if hard_fail else 0)

if __name__ == "__main__":
    main()
