---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: tasks
---

# Tasks

- [x] `receipt_line` and the gate summary join with ` - ` (`src/sessions.rs`, `src/gates.rs`).
- [x] `claim_of` reads both joins (`src/journal.rs`).
- [x] Unit test in `src/journal.rs` for a line with the earlier ` — ` join, and the updated expectations in `tests/sessions.rs` and `tests/migrate_evidence.rs`.
- [x] REQ-history-006, REQ-records-011 and REQ-cli-061 text and acceptance criteria; `docs/cli.md`; CHANGELOG.
- [x] Case in its own `.chough/cases/history.archived-stories.cases.toml`.

Human gates (not checkboxes): scope approval, implementation review, merge.
