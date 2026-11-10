---
change: append-only-event-corrections-with-chough-correct-never-rewriting-history-167
artifact: testing
---

# Testing

Done: RED regressions for appending a correction to a recorded event, surfacing it in history show/next/status and for an archived session's next_action, and proof that the original event bytes and digests are unchanged.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-015 (added) | tests/history.rs (`a_correction_appends_and_is_shown_without_rewriting_history`) |
| REQ-cli-021 (modified) | tests/history.rs (the same test's next/status assertions) |
| REQ-reporting-001 (modified) | tests/timing.rs (existing kind-handling coverage; the correction kind joins the no-segment set) |
