---
change: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
artifact: testing
---

# Testing

Each of these fails before the change:
- tests/cases.rs `cases_split_per_change_load_together_and_ids_stay_unique`: `cli.cases.toml` and `cli.extra.cases.toml` load together, and a case ID defined in both files is refused.
- tests/vault.rs `index_check_names_stale_files_and_passes_after_a_rebuild`: after `library index`, the check passes. It fails naming `INDEX.md` after that file is edited, and passes again after a rebuild. A CRLF copy of a current note passes.
- tests/bugs.rs `two_edits_of_one_bug_merge_and_a_real_disagreement_stays_a_conflict`: a base and two edited copies of one bug. One adds a note and verifies it; the other adds a note and an observation. They merge with both notes, the observation, the verified status and the later update time. Two different statuses exit nonzero and leave ours unchanged.

A scripted simulation on Linux: two branches from this branch's tip each start a session, add a case in their own file, record a case run, edit one shared bug, rebuild the vault, append a ledger line and add a CHANGELOG bullet. With both drivers configured, `git merge` is clean, and after `chough library index` the check passes. `fledge lanes run verify` passes on Linux, including the new knowledge step.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-047 | tests/cases.rs `cases_split_per_change_load_together_and_ids_stay_unique`, tests/vault.rs `index_check_names_stale_files_and_passes_after_a_rebuild`, tests/bugs.rs `two_edits_of_one_bug_merge_and_a_real_disagreement_stays_a_conflict` |
