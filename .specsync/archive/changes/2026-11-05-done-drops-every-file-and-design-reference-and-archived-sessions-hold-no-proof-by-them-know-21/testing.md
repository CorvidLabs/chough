---
change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
artifact: testing
---

# Testing

## tests/reduce.rs

- `done_drops_every_file_and_design_reference`:
  - no reset is recorded;
  - the checkpoint names a cold loose folder file, a source file and a design file, and observation u3;
  - done reports `dropped`, with both files and the design file;
  - it empties both lists and keeps u3;
  - the cold folder is trimmed.

  It fails first: the step reports `skipped`.
- `an_archived_session_file_reference_protects_nothing`:
  - in sessions mode, an archived session names a cold loose folder by file and design reference;
  - the done trim removes the folder;
  - the archived session file is unchanged.

  It fails first: the folder is protected by `archived session old`.
- Existing tests:
  - `done_drops_checkpoint_references_from_closed_cycles` and `an_unreadable_record_keeps_its_reference_and_refuses_the_drop` still pass;
  - `a_checkpoint_folder_reference_protects_loose_files_and_observation_sets` now runs through `library archive --apply`, with every assertion kept.

## Lanes

`fledge lanes run verify` passes on Linux.
