---
change: pin-every-history-and-inbox-bound-at-exactly-its-value-154-ch-19
artifact: context
---

# Context

Issue #154, the audit's CH-19 ("Sessions/history" row, unverified): "Every history capacity, orphan, backfill, prune and inbox path has independent end-to-end evidence."

A mutation check loosened each of the 11 history and session bounds by one and ran the history, sessions, reduce and unit tests. Five mutations survived: `RECENT_BYTES` (512 KiB), `CYCLE_BYTES` (16 MiB), `RECEIPT_BYTES` (1 MiB), `INBOX_LIMIT` (16,384 bytes) and `MAX_INBOX` (200). The other six were already caught. The script is `_worktrees/.prof105/mutate-ch19.sh`.

These are test gaps only: each bound is enforced, but no test pinned its exact value.
