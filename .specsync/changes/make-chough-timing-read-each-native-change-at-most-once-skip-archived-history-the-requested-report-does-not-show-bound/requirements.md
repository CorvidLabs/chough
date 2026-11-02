---
change: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound
artifact: requirements
---

# Requirements

Adds REQ-cli-024. REQ-cli-003, REQ-cli-020 and REQ-cli-021 are unchanged: the
open checkpoint PR #29 modifies REQ-cli-001, 006, 007, 013, 016 and 019, so a new
requirement keeps the two deltas disjoint.

- REQ-cli-024, timing native reads: request-local deduplication, explicit history
  selection, a whole-report native budget for `chough timing` only, and an opt-in
  stderr trace of native reads and timing phases. Trace: LIFE-10, LIFE-3, LIFE-6,
  LIFE-7.

Issue: https://github.com/CorvidLabs/chough/issues/32 (first slice; not the whole
issue).
