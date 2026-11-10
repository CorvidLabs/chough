---
change: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-history-006 | New regression `a_finished_session_with_an_open_gate_stays_open_on_switch` in tests/sessions.rs: one gated finalized session stays open with the recovery named, one ungated finalized session is archived unchanged; existing switch/inbox/doctor suites unchanged and passing |

RED observed by stashing the src fix: the regression fails. GREEN with the fix. Full suite: `fledge run test` green on macOS arm64.
