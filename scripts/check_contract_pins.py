#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Verify vendored immutable compatibility files without network access."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
checked = 0
for name in ("agent-conversation-v1", "codex-app-server-0.160.0"):
    directory = ROOT / "contracts" / name
    lock = json.loads((directory / "lock.json").read_text())
    for relative, digest in lock["files"].items():
        target = (directory / relative).resolve()
        if not target.is_relative_to(directory) or not target.is_file():
            raise SystemExit(f"invalid pinned file: {name}/{relative}")
        if hashlib.sha256(target.read_bytes()).hexdigest() != digest:
            raise SystemExit(f"pin mismatch: {name}/{relative}")
        checked += 1
print(f"verified {checked} pinned files")
