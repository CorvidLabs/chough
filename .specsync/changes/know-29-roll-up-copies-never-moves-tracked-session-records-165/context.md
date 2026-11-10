---
change: know-29-roll-up-copies-never-moves-tracked-session-records-165
artifact: context
---

# Context

Issue #165 (Codex's P2 on CorvidLabs/spec-sync#941, deferred here): the KNOW-29 roll-up moved archived session records into the vault; where `knowledge/` is untracked, tracked records left the tree. Leif's decision (2026-11-10, recorded by the coordinator on the issue): "copy, never move" — tracked session records stay put; the vault gets a copy or a link.

Consequence flagged for acceptance: a rolled session stays listed by `session list` and stays a retention reference source, since its record no longer leaves the archive directory.
