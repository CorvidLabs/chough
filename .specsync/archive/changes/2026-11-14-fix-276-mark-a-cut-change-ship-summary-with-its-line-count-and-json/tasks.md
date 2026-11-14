---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: tasks
---

# Tasks

- [x] Add `short_summary` in `src/change.rs` and use it in `print_summary`.
- [x] Add the unit test
  `change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines`.
- [x] Add the case `TC-cli-change-ship-cut-summary-names-the-cut-and-json` to
  `.chough/cases/cli.change-ship.cases.toml`.
- [x] Update `specs/cli/requirements.md` (REQ-cli-059), `docs/cli.md` and
  `CHANGELOG.md`.
- [x] `fledge run fmt`, `fledge run test`, `fledge run lint` and the case check
  all pass.

Human gates and delivery, as plain text: a human approves this definition with
`specsync change approve`, then `specsync change check`, `specsync change ship`,
CI, review and merge. None of those ran here: the worktree has no open session
and no human authority, so no approval was recorded.