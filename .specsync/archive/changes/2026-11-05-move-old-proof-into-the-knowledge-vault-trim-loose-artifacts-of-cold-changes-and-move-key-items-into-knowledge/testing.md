---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: testing
---

# Testing

New tests in tests/reduce.rs, each failing first:
- Loose artifacts of a cold change are trimmed (listed by a cold archive, or committed in
  its cycle), and current or bug-held ones stay.
- A closed bug releases its loose fixture.
- Key items move into the vault (a manual observation with a screenshot, and an older
  failed run's log), and a routine record is removed.

The existing reduce, evidence, librarian and vault tests keep passing. The trim is
measured on this repository, and `fledge lanes run verify` runs on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `loose_artifacts_of_a_cold_change_are_trimmed_and_current_or_bug_held_ones_stay`, `a_closed_bug_releases_its_loose_fixture`, `key_items_of_a_cold_change_move_into_the_vault`; tests/evidence.rs `archive_summary_previews_cold_archive_proof_and_keeps_protections`, `archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change`; src/attachments.rs unit tests |
