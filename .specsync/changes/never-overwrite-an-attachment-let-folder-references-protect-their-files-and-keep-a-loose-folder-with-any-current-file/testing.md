---
change: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
artifact: testing
---

# Testing

Each new test in tests/reduce.rs fails before the fix:

- `an_attachment_holding_other_bytes_keeps_both_files`: a committed attachment with other bytes; the source stays, both are unchanged, and the set is protected with the reason.
- `an_attachment_written_during_the_apply_refuses_it`: a Git stand-in writes the destination after planning; the apply is refused, the source is restored and the other agent's bytes stay.
- `an_identical_attachment_in_place_survives_a_refused_apply`: the attachment is already in place with the same bytes; the apply is refused afterwards (validity changes), and the attachment is still there.
- `an_open_bug_folder_fixture_protects_its_files`: an open bug's fixture is a folder; its cold-listed log stays.
- `a_checkpoint_folder_reference_protects_loose_files_and_observation_sets`: a checkpoint reference to a folder keeps a loose log in it and a superseded run whose log is in it.
- `a_loose_folder_with_a_current_file_stays_whole`: an old cold-listed log beside a log committed after the newest archive; both stay, and the folder is reported protected.

`fledge lanes run verify` runs on Linux, the trim reruns on this repository at closeout, and CI runs on Windows, macOS and Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `an_attachment_holding_other_bytes_keeps_both_files`, `an_attachment_written_during_the_apply_refuses_it`, `an_identical_attachment_in_place_survives_a_refused_apply`, `an_open_bug_folder_fixture_protects_its_files`, `a_checkpoint_folder_reference_protects_loose_files_and_observation_sets`, `a_loose_folder_with_a_current_file_stays_whole` |
