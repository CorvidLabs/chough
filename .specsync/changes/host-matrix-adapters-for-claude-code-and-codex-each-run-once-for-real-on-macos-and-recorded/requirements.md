---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: requirements
---

# Requirements

Adds REQ-cli-067 (see `deltas/cli.md`), tracing LIFE-21 item 4, LIFE-22 and LIFE-27.

Observable outcomes:

1. `sh tools/host-matrix/selftest.sh` reads a turn that exits 75 as UNAVAILABLE, naming
   the reason, under dash and bash.
2. `integrations/claude-code/adapter.sh` and `integrations/codex/adapter.sh` exist and
   drive their hosts as plain runs.
3. One real macOS leg of each host is recorded as an agent-assisted case run with its
   verdict (PASS, FAIL or UNAVAILABLE) and its receipt, whatever the verdict.
