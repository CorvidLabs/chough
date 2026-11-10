---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: tasks
---

# Tasks

- [x] Amend existing REQ-cli-011 and REQ-cli-017 through one canonical cli delta; allocate no new HI or requirement IDs
- [x] Add the manual-record and evidence-add response regressions and their two change-specific case definitions; run them against the unfixed implementation and record the failures with one deduplicated local bug
- [x] Return `case` and `observation` from `cases record`, and `observation` from `evidence add`, using the IDs already written to the evidence record
- [x] Document the additive keys in docs/cli.md and add one Unreleased CHANGELOG bullet

Remaining as plain text, not checkboxes (case runs and native steps are never tasks): rerun the focused cases and verify the bug from a later pass, run the full test and verify lanes, then native verification, review and archive before hosted CI.
