---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: tasks
---

# Tasks

- [ ] `receipt_line` and the gate summary join with ` - ` (`src/sessions.rs`, `src/gates.rs`).
- [ ] `claim_of` reads both joins (`src/journal.rs`).
- [ ] Unit test in `src/journal.rs` for a line with the earlier ` — ` join, and the updated expectations in `tests/sessions.rs` and `tests/migrate_evidence.rs`.
- [ ] REQ-history-006, REQ-records-011 and REQ-cli-061 text and acceptance criteria; each module's spec change log; `docs/cli.md`; CHANGELOG.
- [ ] Case in its own `.chough/cases/history.archived-stories.cases.toml`.

Human gates (not checkboxes): scope approval, implementation review, merge.
