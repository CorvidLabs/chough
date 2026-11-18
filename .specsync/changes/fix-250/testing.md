---
change: fix-250
artifact: testing
---

# Testing

`tests/doctor.rs::doctor_decision_fixes_need_a_reason_and_record_no_human_decision`
is extended. After `chough doctor --fix --close <id> --reason "<text>"` it reads
the archived story of the session whose stale gate was withdrawn and expects the
`gate-closed` entry to hold `  - withdrawn: <text>`, and expects each decision
finding in doctor's JSON to give `decision_reason` and each other finding not
to. It fails on main, where the entry has no such field. The test's existing
checks stay: withdrawing records no human decision.

`tests/doctor.rs::a_stale_gate_reason_that_leaks_is_refused_or_redacted` (added after
the tester's finding on #345): a reason holding a credential is refused as
`refused:leak` with nothing written, and with `--redact` it is written redacted in
the story and in doctor's JSON. It fails without the leak guard.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-012 | the test above and the case in `.chough/cases/history.stale-gate-reason.cases.toml` |
| REQ-cli-030, REQ-records-004 | the same test: the entry is written and read back with its `withdrawn` field |
