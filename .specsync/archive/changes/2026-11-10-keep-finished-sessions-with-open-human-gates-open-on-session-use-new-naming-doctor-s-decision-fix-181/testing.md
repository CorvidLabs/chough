---
change: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-history-006 | `TC-history-switch-keeps-gated-finished-sessions-open` binds `a_finished_session_with_an_open_gate_stays_open_on_switch` in tests/sessions.rs: a gated finalized session and a gated done session stay open byte for byte with their gate counts and recoveries, and an ungated finalized session is archived unchanged |

Failing-first: the regression is committed before the fix and run there through the case (failed observation linked to the #181 bug); the same case passes after the fix. Existing switch, inbox and doctor suites run unchanged in the full `fledge run test` suite.
