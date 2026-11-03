---
change: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to
artifact: tasks
---

# Tasks

- [x] Failure-first regression in tests/stages.rs and case TC-cli-delta-guard-refusals-name-the-agent-actor
- [x] Record the failing observation and a local bug before the fix
- [x] Reword the two refusal remedies in src/stage.rs to match docs/cli.md
- [x] Update the two existing assertions that quote the old remedy, changing only the quoted text
- [x] Focused case passes after the fix; `fledge lanes run verify` runs after the scoped check, with output kept outside the repository
