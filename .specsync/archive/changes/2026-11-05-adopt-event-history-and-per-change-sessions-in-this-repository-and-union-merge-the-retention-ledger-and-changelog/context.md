---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: context
---

# Context

Leif asked for unique sessions so pull requests stop conflicting (2026-11-04), and chose "Adopt + new criterion" (2026-11-05). DRIVE-12 was captured from that choice. This repository still runs Chough in legacy mode: one `.chough/session.json` checkpoint and one appended `.chough/transitions.jsonl`. Every pull request rewrites both, so parallel branches conflict on them almost every time. Chough already ships per-change sessions (REQ-history-006, DRIVE-3) and immutable event history (REQ-history-007). This change adopts both here, on main before other branches diverge, so later pull requests each write their own files.
