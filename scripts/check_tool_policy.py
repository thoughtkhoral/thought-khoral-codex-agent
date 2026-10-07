#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Check exact catalog provenance and only the approved tool-field modifications."""
import hashlib,json,pathlib
root=pathlib.Path(__file__).resolve().parents[1]/'contracts/codex-app-server-0.160.0'
original=(root/'upstream-models.json').read_bytes()
assert hashlib.sha256(original).hexdigest()=='fd219bd9f061278275f528939f82f54d2eb97df4b25c23b022adbe48813d920b'
expected=json.loads(original)
for model in expected['models']:
 model.update(shell_type='disabled',apply_patch_tool_type=None,experimental_supported_tools=[],tool_mode='direct',supports_search_tool=False,multi_agent_version=None)
assert json.loads((root/'restricted-models.json').read_text())==expected
print('exact pinned catalog: 11 descriptors; only six approved tool fields changed')
