---
change: a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18
artifact: testing
---

# Testing

New tests in tests/subdirectory.rs. Each uses a repository whose Chough project is in `app/`, plus a README outside the project.
- `a_project_in_a_subdirectory_records_evidence`:
  - `cases run` in `app/` passes, and its record has the repository's HEAD and is not dirty;
  - `evidence add` succeeds;
  - `status` reports a known snapshot at HEAD.
- `a_subdirectory_snapshot_covers_only_the_project`:
  - editing the README and adding an untracked file outside `app/` leave the snapshot identical;
  - editing `app/src/input.txt` makes it dirty and changes the content fingerprint.
- `a_project_has_the_same_snapshot_at_the_root_and_in_a_subdirectory`: the same project, at a repository root and in `app/`, has the same content fingerprint and dirty state.

All three fail first with "--root must be the Git repository root for evidence". The existing snapshot and evidence tests stay unchanged. A case binds the tests. `fledge lanes run verify` passes on Linux.
