---
change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
artifact: tasks
---

# Tasks

- [x] A failing test in `tests/migrate.rs`: after the apply, doctor reports `stale_gates` and `finished_session`, and the preview's follow-ups and the receipt's `## Follow-up` name each, the stale gates with their text and `chough doctor --fix --reason`.
- [x] One shared session check in `src/doctor.rs`, called by doctor and by migrate's plan in `src/migrate.rs`; `docs/cli.md` and the CHANGELOG.
- [x] Red then green recorded through Chough, and the full local suite.

Human gates and delivery, as plain text: Leif's review through `chough change ship`, then CI, then his merge.
