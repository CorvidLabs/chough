# Lesson bundle: a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A project in a subdirectory of its Git repository records evidence against its own files (170, DRIVE-18)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/subdirectory.rs, specs/cli/cli.spec.md, docs/cli.md, hi/drive.md, CHANGELOG.md
- **Acceptance**: DRIVE-18 (Leif, relayed by orc 2026-11-06, 'Capture both'; chough#170), in the new REQ-cli-056: in a project in a subdirectory of its Git repository, cases run, evidence add and status work from that directory. The snapshot covers the project's own files with the repository's HEAD. Edits outside the project change nothing, and edits inside make it dirty. The same project has the same content fingerprint at the root and in a subdirectory. The '--root must be the Git repository root' refusal is gone. The three new tests in tests/subdirectory.rs fail first; docs/cli.md (the line-5 limitation) and CHANGELOG say so; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `8568be3fb0a07faf386629aafb353bebb335dcc2`
- Base commit: `167790489735b3e6a88ec21a20bb2530d4c96138`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #170, found by the independent auditor in `_audits/specsync-chough-interim-20261106`, using the `subdirectory-ts-v2` fixture with the project in `app/`.
- `chough cases run` in `app/` refuses: "--root must be the Git repository root for evidence".
- The advice, `--root <git root>`, fails: "run chough init first", because the checkpoint is in `app/`.
- `status` reports the snapshot as unknown, and `evidence add` and `cases record` refuse the same way.

This is the Chough counterpart of SpecSync's D2 (spec-sync #879, fixed in #890). `docs/cli.md` documented it as a limitation. Leif captured DRIVE-18 ("Capture both").

## From the change's design.md

# Design

## Approach

In `evidence::snapshot` (`src/evidence.rs`), instead of refusing when `git rev-parse --show-toplevel` differs from the project root:
- **Prefix:** take `git rev-parse --show-prefix`, which is empty at the top level, and require the project root to be the top level joined with that prefix.
- **Files:** `git ls-files --cached --others --exclude-standard -z`, run in the project root, already lists only the files under it, relative to it, so the hashing and the `.chough/` and `target/` filters work unchanged.
- **Dirty:** `git status --porcelain --untracked-files=all -- .` limits the dirty check to the project. Its paths are relative to the repository root, so the prefix is stripped before `fingerprint_key`; a path outside the prefix is ignored.
- **Revision:** the repository's `HEAD`, as before.

At the top level the prefix is empty, so every command and byte is as before.

## Risks

- **Renamed paths** in porcelain output (`R  old -> new`) keep today's handling: the text after the status code is checked as a whole, as before.
- **Paths outside the project** (for example `../x`) never appear, because `ls-files` and the pathspec `.` are scoped to the project.
- **Out of scope:** a project outside any Git repository is unchanged; Git's own error still applies.

## Constraints consulted

DRIVE-18, REQ-cli-056 (new), SpecSync D2.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
