---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: testing
---

# Testing

`tests/sessions.rs` and `tests/migrate_evidence.rs` expect a What was verified line joined with ` - ` at archiving and after migration. A unit test in `src/journal.rs` gives `claim_of` a line joined with ` — ` and one joined with ` - ` and expects the same claim from both; it fails without the legacy read. A case in `.chough/cases/history.archived-stories.cases.toml` binds the archiving test to REQ-history-006. Checks: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge lanes run verify`, and `chough change check`.
