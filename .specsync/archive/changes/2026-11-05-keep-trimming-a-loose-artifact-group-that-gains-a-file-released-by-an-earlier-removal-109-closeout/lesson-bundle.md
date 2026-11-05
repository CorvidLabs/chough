# Lesson bundle: keep-trimming-a-loose-artifact-group-that-gains-a-file-released-by-an-earlier-removal-109-closeout

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep trimming a loose artifact group that gains a file released by an earlier removal (109 closeout)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/reduce.rs
- **Acceptance**: A loose artifact group that gains a file because an earlier removal in the same apply released it is still trimmed: its planned files are removed and the released file stays; the re-check still refuses when a planned loose file becomes referenced or protected. The new test fails first, the trim on this repository then completes, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `6e0a8357cb0d6e048826edc6c67aa26b9dce6b2c`
- Base commit: `6009d42756364fd1c338b215ac8539c1b2a0d0b0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

#109's closeout ran the move into done on Chough's own repository, and the trim was refused partway: "protections changed during removal (artifact:handoff-cold-start: its files changed)". It restored the 172 files it had moved or removed (KNOW-10). Removing the observation records that shared a log inside the `handoff-cold-start` folder made that log loose. The folder's loose group then listed it before its planned files, and the per-file re-check required the first file to be unchanged. That requirement fits an observation set, whose record is removed last, but not a loose group. For a loose group, the re-check now requires only that every planned file still present is still loose and the group still unprotected. Only planned files are ever removed, so the released file stays.

## From the change's testing.md

# Testing

tests/reduce.rs `a_loose_folder_that_gains_a_released_file_is_still_trimmed`: a folder whose a-shared.log is referenced by two eligible runs, and whose z-loose.log is loose and listed by cold a1. The trim must succeed, remove z-loose.log and keep a-shared.log. It fails first with the same refusal. tests/reduce.rs `a_loose_file_protected_during_the_apply_refuses_the_trim` guards the other side: an open bug that names a planned loose file partway through the apply refuses the trim and restores both files. `fledge lanes run verify` runs on Linux, and the trim then runs on this repository at closeout.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `a_loose_folder_that_gains_a_released_file_is_still_trimmed`, `a_loose_file_protected_during_the_apply_refuses_the_trim` |

## Where these lessons go

- `specs/cli/context.md`
