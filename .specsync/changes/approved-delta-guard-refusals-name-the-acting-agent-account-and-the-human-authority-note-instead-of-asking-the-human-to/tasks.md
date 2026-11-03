---
change: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to
artifact: tasks
---

# Tasks

- [ ] Failure-first regression in tests/stages.rs and case TC-cli-delta-guard-refusals-name-the-agent-actor
- [ ] Record the failing observation and a local bug before the fix
- [ ] Reword the two refusal remedies in src/stage.rs to match docs/cli.md
- [ ] Update the two existing assertions that quote the old remedy, changing only the quoted text
- [ ] Focused case passes after the fix; `fledge lanes run verify` passes
