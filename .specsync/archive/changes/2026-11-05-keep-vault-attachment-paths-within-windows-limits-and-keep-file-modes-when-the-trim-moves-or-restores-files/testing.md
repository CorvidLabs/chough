---
change: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
artifact: testing
---

# Testing

Each new test fails before the fix:

- tests/reduce.rs `key_items_of_a_long_archive_id_move_under_a_short_folder`: cold a1 renamed to a 61-character ID; its screenshot moves under `a1-a-long-<8 hex>/` and the note links it there.
- tests/reduce.rs `a_key_item_whose_attachment_path_is_too_long_stays`: a failure-first log whose attachment path would be 244 characters stays, and `artifact:x` is reported protected with the 220-character reason.
- tests/reduce.rs `a_moved_key_item_keeps_its_mode`: an executable failure-first script is 0755 in the vault.
- tests/reduce.rs `a_refused_apply_restores_files_with_their_modes`: a SpecSync stand-in reports changed validity once an executable loose script is gone; the refused apply restores it as 0755 with its bytes.
- src/attachments.rs `long_archive_ids_get_a_short_stable_folder`: short IDs are kept; long ones are cut at a hyphen (or at 16 characters) with the expected digest.

`commit_aged` gets the fixture's Git identity, so `loose_artifacts_of_a_cold_change_are_trimmed_and_current_or_bug_held_ones_stay` runs on CI runners without one. `fledge lanes run verify` runs on Linux, and the trim reruns on this repository at closeout. Windows CI must then check the branch out.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `key_items_of_a_long_archive_id_move_under_a_short_folder`, `a_key_item_whose_attachment_path_is_too_long_stays`, `a_moved_key_item_keeps_its_mode`, `a_refused_apply_restores_files_with_their_modes`; src/attachments.rs `long_archive_ids_get_a_short_stable_folder` |
