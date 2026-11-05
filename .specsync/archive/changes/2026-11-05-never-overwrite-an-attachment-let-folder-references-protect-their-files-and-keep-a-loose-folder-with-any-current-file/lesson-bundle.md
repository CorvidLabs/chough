# Lesson bundle: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Never overwrite an attachment, let folder references protect their files, and keep a loose folder with any current file (109 review P1s)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/reduce.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: Moves into the vault never overwrite: an identical attachment is kept and survives a refused apply, a different one keeps both files and protects the set, and one written during the apply refuses it. A folder named by the checkpoint or an open bug keeps the loose files and observation sets inside it. A loose folder with any current file stays whole. The new tests fail first, fledge lanes run verify passes on Linux, and the trim reruns on this repository at closeout.

## Evidence

- Verification commit: `c402031bffd49fe40845984b3e36cde32c75525c`
- Base commit: `d165b0a20d05f29c884e6970a30fb50b88c950fd`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif's review agent left three P1 threads on #109 at 6e0a835 (2026-11-05), each reproduced:

1. **An attachment is overwritten.** The move uses `fs::write`, which truncates an existing destination and then registers it as a copy this apply made. A committed `knowledge/attachments/a1/evidence/m9.json` holding another agent's proof was replaced. A rollback would also delete an identical attachment that was already there.
2. **A folder fixture does not protect its files.** Bug fixtures and checkpoint references are matched only against exact file paths. An open bug whose fixture is the folder `.chough/artifacts/open-repro` lost `open-repro/out.log` to the trim. The observation-set rules match references the same way.
3. **A cold sibling decides the owner of a whole folder.** One cold-listed file made a loose folder cold, and the first-commit check was skipped for listed folders. So `reused-folder/current.log`, committed after the newest archive, was removed along with `a-old.log`.

Leif chose "Record both, then merge" (2026-11-05). The previous change, for the Windows paths and file modes, is archived, and the move into done waits for this one, so no trim runs with these bugs.

## From the change's design.md

# Design

- **Named through a folder.** A helper `naming(map, key)` yields the sources recorded for the key and for each of its ancestors, split at `/` on the case-folded canonical key that `file_key` gives, so `a/b` never matches `a/bc`. The loose-group protections (checkpoint and bug sources, open bug fixtures) and the observation-set rules (named reasons, and artifacts kept out of removal) use it.
- **Ownership per file.** Each file of a loose group is owned by the archives that list it, or else by `owner_by_cycle` of its own first commit. A group whose files are all current or uncommitted is skipped as before. A group with some such files is reported protected, naming them. Hot owners protect as before. `listed_by` stays the listing archives; `attributed` is set when any file was owned by cycle time.
- **Create-only moves.** `summary_plan` compares each planned move with what is at its destination. Different bytes protect the candidate with the reason; the same bytes mean nothing needs copying. The apply opens the destination with `create_new`. If something appeared meanwhile, the same bytes are kept without registering a copy, and different bytes fail the apply, which restores the originals and leaves that file alone. Only copies this apply created are deleted on rollback.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
