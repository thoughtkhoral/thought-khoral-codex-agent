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

[Dependency evidence](docs/dependency-evidence.json) records all 139 locked
registry packages, SPDX declarations, selected permissive license alternatives,
crate checksums and available packaged legal-file hashes. Packages carrying
Unicode-3.0, MIT-0 and Zlib terms retain those terms. No A2A client, gateway
implementation or SQLx is included in this library unit. Task 5 must package
applicable dependency licenses/copyright notices with any distributed binary
or image. This source deliverable does not include third-party crate sources
or a Codex executable.
