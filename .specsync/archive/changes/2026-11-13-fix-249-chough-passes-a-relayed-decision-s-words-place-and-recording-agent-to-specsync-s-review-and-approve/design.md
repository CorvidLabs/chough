---
change: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
artifact: design
---

# Design

`Step::arguments` in `src/humans.rs` takes the relay and appends `--words`, `--where`
and `--recorded-by` to both `specsync change approve` (after `--actor` and `--note`)
and `specsync change review` (after `--reviewer` and `--verdict`). The callers in
`src/checkpoint.rs` (gate closes, `chough gate record`) and `src/change.rs` (`chough
change ship`) pass the relay they already checked. The approval note keeps its text.
No new flag on any Chough command. REQ-records-008 names the new arguments, and
REQ-cli-059's ship criterion says the review gets the relay. `docs/cli.md` and the
CHANGELOG's Unreleased section describe it.
