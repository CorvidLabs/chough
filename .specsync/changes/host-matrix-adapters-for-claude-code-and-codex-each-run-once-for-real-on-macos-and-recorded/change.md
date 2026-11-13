---
id: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
state: draft
type: feature
base_commit: 7386391e54c94dff3dabebf58225d618650bd742
---

# Host-matrix adapters for Claude Code and Codex, each run once for real on macOS and recorded

## Intent

Host-matrix adapters for Claude Code and Codex, each run once for real on macOS and recorded

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The self-test reads a turn that exits 75 as UNAVAILABLE naming the reason, on macOS and Linux
- Claude Code and Codex each have an adapter under integrations/<host>/adapter.sh
- One real macOS run of each host is recorded as a manual case with its verdict and receipt, whatever the verdict

## No-spec Rationale

Not applicable
