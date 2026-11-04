---
change: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
artifact: context
---

# Context

On Raven on 2026-11-03, stages ran outside Chough (for example in subagents). The only
way to reflect them afterwards was to open and close gates now, so the transition log
showed the time of recording with "recorded after the fact" in the reason, and `timing`
reported the wrong durations. Leif's brief t811u item 5 asks for `chough stage
backfill` (or similar) that records a past gate with its original time, the approval
evidence (commit, quoted choice) and a backfilled flag that `timing` reports
separately (DRIVE-5).

The hard rule holds (parent, 6:03 PM MST): backfill never creates a human approval or
review record, and it refuses if native state does not support the gate. The requirement is ADDED REQ-cli-036, kept short because the cli requirements
companion is near its 65,536-byte bound.
