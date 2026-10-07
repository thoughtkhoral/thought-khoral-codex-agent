#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Check repository foundation and specification-derived Markdown locally."""

import re
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
REQUIRED = (
    "README.md", "AGENTS.md", "CONTRIBUTING.md", "LICENSE", "NOTICE",
    ".ai/specs/README.md", ".ai/specs/what/repository-foundation.md",
    ".ai/specs/what/public-documentation.md", ".ai/specs/how/repository-foundation.md",
    ".ai/specs/decisions/001-governance-and-license.md",
    ".github/CODEOWNERS", ".github/workflows/documentation.yml",
    "docs/README.md", "docs/architecture.md",
)
PARENT = "https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md"
LINK = re.compile(r"\]\(([^)]+)\)")
SOURCE = re.compile(r"\]\((?:\.\./)*\.ai/specs/[^)]+\)")
SKIP = {".git", ".codex", "sessions", "archived_sessions", "data", "secrets",
        "node_modules", "target", "dist", "__pycache__", "licenses"}


def main() -> int:
    failures = []
    for name in REQUIRED:
        path = ROOT / name
        if not path.is_file() or not path.read_text().strip():
            failures.append(f"missing or empty required file: {name}")
    for name in ("what", "how", "decisions"):
        if not list((ROOT / ".ai/specs" / name).glob("*.md")):
            failures.append(f"missing specification area: {name}")
    index = ROOT / ".ai/specs/README.md"
    if not index.is_file() or PARENT not in index.read_text():
        failures.append("specification index must link to root governance")
    license_path = ROOT / "LICENSE"
    if not license_path.is_file() or "Version 2.0, January 2004" not in license_path.read_text():
        failures.append("LICENSE must contain Apache License 2.0")

    checked = 0
    for path in sorted(ROOT.rglob("*.md")):
        relative = path.relative_to(ROOT)
        if any(part in SKIP for part in relative.parts):
            continue
        text = path.read_text()
        if (len(relative.parts) == 1 or relative.parts[0] == "docs") and not SOURCE.search(text):
            failures.append(f"{relative}: documentation must link its governing local specification")
        fence = None
        for number, line in enumerate(text.splitlines(), 1):
            stripped = line.lstrip()
            marker = "```" if stripped.startswith("```") else "~~~" if stripped.startswith("~~~") else None
            if marker:
                if fence is None:
                    fence = marker
                elif fence == marker:
                    fence = None
                continue
            if fence:
                continue
            for match in LINK.finditer(line):
                raw = match.group(1).strip()
                target = raw.split(">", 1)[0][1:] if raw.startswith("<") else raw.split(" ", 1)[0]
                parsed = urlsplit(target)
                if parsed.scheme or target.startswith("//") or not parsed.path:
                    continue
                checked += 1
                destination = (path.parent / unquote(parsed.path)).resolve()
                if not destination.is_relative_to(ROOT) or not destination.exists():
                    failures.append(f"{relative}:{number}: missing or external relative target: {target}")
    for failure in failures:
        print(f"FAIL: {failure}")
    print(f"checked repository foundation and {checked} local Markdown links; {len(failures)} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
