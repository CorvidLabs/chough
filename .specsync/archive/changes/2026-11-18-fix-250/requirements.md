---
change: fix-250
artifact: requirements
---

# Requirements

- REQ-history-012 (`deltas/history.md`): the `stale_gates` fix writes its reason as the
  `withdrawn` field of each withdrawn gate's `gate-closed` entry, and an applied
  decision fix echoes the reason as `decision_reason`. The reason is checked for leaks
  like any written value.
- REQ-cli-030 (`deltas/cli.md`) and REQ-records-004 (`deltas/records.md`): one clause
  each, pointing at that field, because those modules own the code that writes the gate
  entry and that reads and writes `session.md`.
