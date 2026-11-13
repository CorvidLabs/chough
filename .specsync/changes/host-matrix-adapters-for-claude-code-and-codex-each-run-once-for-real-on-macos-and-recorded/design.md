---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: design
---

# Design

Nothing visual. The adapters follow the contract in `tools/host-matrix/README.md`:

- `host_check` requires the CLI, and for Codex `codex login status`.
- `host_version` names the version and any pinned model.
- `host_turn` runs one turn and returns 75 when the log shows a missing login or a usage
  limit.

In `run.sh`, a turn status of 75 writes `turn <n>: the host cannot run now (<reason>)`
and ends the leg as UNAVAILABLE. Every other non-zero status stays a failed check.
