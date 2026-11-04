---
change: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88
artifact: testing
---

# Testing

- tests/librarian_trim.rs, with five stub archives (two cold) and a checkpoint at accept:
  - `entering_done_trims_cold_archived_proof_by_default` (REQ-cli-041): the move
    succeeds, `applied.librarian_trim.status` is `trimmed`, the cold observation is
    pruned and ledgered, its authority names KNOW-13.
  - `retention_manual_opts_out_of_trimming_at_done` (REQ-cli-041): `skipped`, nothing
    removed.
  - `a_librarian_off_project_is_not_trimmed_at_done` (REQ-cli-041): `skipped`.
  - `an_open_native_change_keeps_proof_and_the_move_still_succeeds` (REQ-cli-041):
    `refused` naming the open change; done is recorded; nothing removed.
- Each fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-041 | tests/librarian_trim.rs `entering_done_trims_cold_archived_proof_by_default`, `retention_manual_opts_out_of_trimming_at_done`, `a_librarian_off_project_is_not_trimmed_at_done`, `an_open_native_change_keeps_proof_and_the_move_still_succeeds` |
