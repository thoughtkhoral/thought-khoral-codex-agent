#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Check retained legal texts against the reviewed locked dependency inventory."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
evidence=json.loads((ROOT/'docs/dependency-evidence.json').read_text())
assert hashlib.sha256((ROOT/'Cargo.lock').read_bytes()).hexdigest()==evidence['cargoLockSha256'], 'dependency inventory requires review after lock changes'
count=0
for package in evidence['packages']:
    for name,digest in package['upstreamLegalFileHashes'].items():
        path=ROOT/'licenses'/f"{package['name']}-{package['version']}"/name
        assert hashlib.sha256(path.read_bytes()).hexdigest()==digest, str(path)
        count+=1
assert (ROOT/'licenses/ratatui-codex/LICENSE').is_file()
print(f"verified {count} retained legal files for {len(evidence['packages'])} locked crates and Ratatui")
