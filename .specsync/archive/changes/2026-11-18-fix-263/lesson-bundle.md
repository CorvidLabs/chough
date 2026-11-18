# Lesson bundle: fix-263

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 263
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/migrate.rs, tests/migrate_knowledge.rs, .chough/cases/records.migrate-case-colliding-target.cases.toml, changelog.d/2026-11-18-fix-263.md
- **Acceptance**: On a case-sensitive file system, when knowledge/index.md already exists with other bytes beside the 0.5.x knowledge/INDEX.md, chough migrate --apply exits non-zero with refused:target-exists naming knowledge/index.md, and no file is written, removed or changed, so no receipt exists. On a case-insensitive file system, where the two names are one file, the apply still migrates as before.

## Evidence

- Verification commit: `c62201a0ef85db42ad357377db73df2198f3f8d9`
- Base commit: `be6c9f126e448bd33945a52261d55db2a9677d3d`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #263, found by a swarm stress test on Debian and reproduced by a second
tester. A 0.5.x project has `knowledge/INDEX.md`; the migration plan removes it
and writes `knowledge/index.md`. On a case-sensitive file system a separate
`knowledge/index.md` with other bytes existed before the apply. The apply did
not refuse. It wrote the other planned files and the receipt, removed every
0.5.x file, and only then failed on `knowledge/index.md` with "appeared while
migrating". The receipt claims bytes that were never written, and a second
apply reports nothing to do.

The cause is the pre-write check. It skips a target that differs only in case
from a planned removal, because on a case-insensitive file system reading
`knowledge/index.md` returns the old `INDEX.md`. On a case-sensitive file system
the two are separate files, and the skip let the junk file through.

REQ-records-009 already requires the refusal: the apply "SHALL refuse, writing
nothing more, when a file it would write already exists with other bytes". No
requirement text changes.

## From the change's testing.md

# Testing

`tests/migrate_knowledge.rs::a_separate_file_at_a_case_colliding_target_refuses_before_any_write`
puts a junk `knowledge/index.md` beside the 0.5.x `knowledge/INDEX.md` and
applies the plan. It expects `refused:target-exists` naming the file and a tree
that is byte for byte unchanged. On a case-insensitive file system the state
cannot exist, so the test says so and stops; it runs for real on Linux, in the
`chough-linux:3` image locally and in CI.

A unit test, `two_names_differing_only_in_case_are_one_file_only_when_the_folder_lists_one`,
covers the file-system check on both kinds of file system.

The existing case-only rename and interrupted-apply tests still pass, so macOS
migrations are unchanged.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | the two tests above and the case in `.chough/cases/records.migrate-case-colliding-target.cases.toml` |

## Where these lessons go

- `specs/records/context.md`
