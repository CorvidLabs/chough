---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: docs
---

# Documentation changes

- CLI Purpose: explicitly name eligible captured-byte authority and per-file unsafe rollback continuation, with no new public command/flag/schema.
- History Error Cases: add the two observable refusal/recovery rows in deltas/history.md. Existing requirement bodies, FRD wording and strict split digest remain unchanged because REQ-history-004 already requires both guarantees.
- History testing companion: bind the capture and both symlink variants through a new per-change case file. The existing case definitions and assertions stay intact.
- History context: record a verified lesson from the two failure receipts: preparation must validate authority against eligible fingerprints, and rollback path validation belongs inside per-file recovery containment. State source, scope, tested variants and limits.
- Native context/research/testing retain issue, PR, thread, predecessor and exact receipt links. Do not claim a passing correction or fresh human review before those happen.
