# Third-party sources

Governing source: approved [runtime design](.ai/specs/how/codex-chat-agent.md).

The stable generated Codex 0.160.0 schemas are unmodified. Their upstream
[Apache license](contracts/codex-app-server-0.160.0/LICENSE) and
[notice](contracts/codex-app-server-0.160.0/NOTICE) are retained. Their provenance
and hashes are in [the native lock](contracts/codex-app-server-0.160.0/lock.json).
The generator used an isolated home and no experimental protocol option.

The published conversation schemas and synthetic fixtures are unmodified
compatibility artifacts owned by ThoughtKhoral contracts. Their immutable
commit, archive digest and individual file hashes are in
[the profile lock](contracts/agent-conversation-v1/lock.json).

[Dependency evidence](docs/dependency-evidence.json) records all 275 locked
registry packages, declarations, selected license alternatives, crate checksums,
legal provenance and retained text hashes. [Legal texts](licenses/) are copied
into the independent image. AND obligations and upstream copyright notices
are retained; for r-efi the upstream AUTHORS and standard Apache terms accompany
the declared Apache alternative. Build-only and inactive-target notices are
included conservatively. A2A server interfaces and published types are reused;
no gateway implementation or patched client is copied.

The image includes the unmodified, checksum-verified Codex CLI 0.160.0 release
binary. Codex LICENSE/NOTICE and the MIT license from its checksum-verified
Ratatui 0.30.2 crate are retained in the image. Debian package copyright files
remain with the runtime base. Run `python3 scripts/check_dependency_notices.py`
when verifying or changing dependencies.
