#!/usr/bin/env python3
"""Verify ROADMAP.md against the test suite.

A milestone is "done" when every checkbox under its `## Mxx` heading is ticked.
Every done milestone must pass its test file, declared in the heading as
`<!-- test: NAME -->` (that is tests/NAME.rs). Milestones with no declared test
are reported as manual. Finished milestones are re-tested on every run, so a
later change that breaks an earlier milestone is caught.

Usage:
    python3 scripts/check.py            # progress + run tests for done milestones
    python3 scripts/check.py --no-run   # progress only

Exit status is 1 if any done milestone fails its tests, otherwise 0.
Standard library only.
"""
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HEADER = re.compile(r"^##\s+(M\d+)\b(.*?)\s*(?:<!--\s*test:\s*(\w+)\s*-->)?\s*$")
BOX = re.compile(r"^\s*-\s*\[([ xX])\]")


def parse(path):
    milestones, cur = [], None
    for line in path.read_text(encoding="utf-8").splitlines():
        m = HEADER.match(line)
        if m:
            cur = {"id": m.group(1), "title": m.group(2).strip(), "test": m.group(3), "done": 0, "total": 0}
            milestones.append(cur)
            continue
        if line.startswith("## "):  # some other section, e.g. Stretch: not tracked
            cur = None
            continue
        b = BOX.match(line)
        if b and cur is not None:
            cur["total"] += 1
            cur["done"] += b.group(1) != " "
    return milestones


def run_tests(name):
    if not (ROOT / "tests" / f"{name}.rs").exists():
        return False, f"tests/{name}.rs does not exist"
    p = subprocess.run(
        ["cargo", "test", "--quiet", "--test", name],
        cwd=ROOT, capture_output=True, text=True,
    )
    return p.returncode == 0, (p.stdout + p.stderr).strip()


def main():
    run = "--no-run" not in sys.argv[1:]
    milestones = parse(ROOT / "ROADMAP.md")
    if not milestones:
        sys.exit("no milestones found in ROADMAP.md")

    rows, failures, warnings = [], [], []
    first_incomplete = None
    for m in milestones:
        complete = m["total"] > 0 and m["done"] == m["total"]
        if not complete and first_incomplete is None:
            first_incomplete = m["id"]
        if m["done"] > 0 and first_incomplete not in (None, m["id"]):
            warnings.append(f"{m['id']} has ticked steps while {first_incomplete} is unfinished (skipping ahead?)")

        if not complete:
            status = "in progress" if m["done"] else "not started"
            icon = "🔨" if m["done"] else "⬜"
        elif not m["test"]:
            status, icon = "done (manual check, no tests yet)", "✅"
        elif not run:
            status, icon = f"done (tests: {m['test']}, not run)", "✅"
        else:
            ok, output = run_tests(m["test"])
            if ok:
                status, icon = f"done, tests pass ({m['test']})", "✅"
            else:
                status, icon = f"TESTS FAIL ({m['test']})", "❌"
                failures.append((m, output))
        rows.append((icon, m, status))

    ticked = sum(m["done"] for m in milestones)
    total = sum(m["total"] for m in milestones)
    pct = 100 * ticked // total if total else 0
    bar = "█" * (pct // 5) + "░" * (20 - pct // 5)

    print(f"\nProgress  {bar}  {ticked}/{total} steps ({pct}%)\n")
    for icon, m, status in rows:
        print(f"  {icon} {m['id']:<4} {m['title']:<34} {m['done']:>2}/{m['total']:<2}  {status}")
    for w in warnings:
        print(f"\n  ⚠️  {w}")
    for m, output in failures:
        tail = "\n".join(output.splitlines()[-40:])
        print(f"\n--- {m['id']} failure output (last 40 lines) ---\n{tail}")
    print()

    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as f:
            f.write(f"### Roadmap: {ticked}/{total} steps ({pct}%)\n\n")
            f.write("| | Milestone | Steps | Status |\n|---|---|---|---|\n")
            for icon, m, status in rows:
                f.write(f"| {icon} | {m['id']} {m['title']} | {m['done']}/{m['total']} | {status} |\n")
            for w in warnings:
                f.write(f"\n> ⚠️ {w}\n")
            f.write("\n")

    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
