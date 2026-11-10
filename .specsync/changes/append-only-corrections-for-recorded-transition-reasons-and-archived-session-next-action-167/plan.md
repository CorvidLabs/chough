---
change: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
artifact: plan
---

# Plan

1. Human approves scope and the command shape in context.md.
2. Draft the history delta (new correction record type, reader surfacing rules).
3. Implement in src/history.rs and src/sessions.rs with RED-first regressions: correction appended, original event bytes unchanged, readers surface the correction.
4. Verify, review, archive and merge per the normal lifecycle.
