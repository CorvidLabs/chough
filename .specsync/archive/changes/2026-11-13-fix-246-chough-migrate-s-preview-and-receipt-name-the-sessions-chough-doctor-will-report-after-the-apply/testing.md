---
change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
artifact: testing
---

# Testing

`tests/migrate.rs`: the adopted 0.5.x fixture plus two sessions with archived native
changes, one waiting on two human gates and one with none. After `chough migrate
--apply`, `chough doctor` reports `stale_gates` and `finished_session`. The
preview's follow-ups and the receipt's `## Follow-up` each name both sessions, and the
stale one with both gates' text and `chough doctor --fix --reason`. Red at the test
commit, green at the fix. Doctor's existing tests still pass unchanged. The full local
suite runs at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | `tests/migrate.rs`: the preview and the receipt name the sessions doctor reports after the apply |
| REQ-cli-055 | `tests/migrate.rs`: after the apply, doctor reports exactly the sessions the follow-ups named |
| REQ-history-012 | doctor's existing tests: its findings are unchanged by the shared check |
