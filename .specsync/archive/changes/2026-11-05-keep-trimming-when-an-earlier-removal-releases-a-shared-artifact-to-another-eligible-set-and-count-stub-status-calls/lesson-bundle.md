# Lesson bundle: keep-trimming-when-an-earlier-removal-releases-a-shared-artifact-to-another-eligible-set-and-count-stub-status-calls

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep trimming when an earlier removal releases a shared artifact to another eligible set, and count stub status calls under a lock now that validity reads run in parallel (108 closeout)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/reduce.rs, tests/evidence.rs
- **Acceptance**: Two eligible observation sets sharing one artifact are both trimmed in one apply, their own artifacts removed and the shared one kept; the per-file re-check still refuses when a planned file becomes shared with a protected set or the set becomes protected; the knowledge fixture stub counts status calls under a lock so hook-timed safety tests are exact under parallel validity reads. The new test fails first, the trim on this repository then completes, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `1807f7b03ebad5ddf44bb7a5009f5a9b7288f6fe`
- Base commit: `f40ac53b8de66ebfc07d51294c8f6c13e22f8ad3`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

#108's closeout ran the move into done on Chough's own repository. It dropped 385 closed-cycle checkpoint references (KNOW-16), and then the trim was refused partway: "protections changed during removal (OBS-product-test-4061...: its files changed)". It restored the 21 files it had removed (KNOW-10 restore).

The cause is in `still_unprotected`, the re-check before every file removal. It required the set's current files to be a subset of the planned files. When two eligible sets share one artifact, neither plan includes that artifact. Removing the first set's record leaves the artifact to the second set alone, so the second set now lists one extra file, and the re-check refused.

A set that gains a file is still safe to trim, because only planned files are removed. The re-check now requires every planned file that is still present to belong to the set alone, with the same record. It still refuses when a planned file becomes shared or the set becomes protected. No requirement text changes.

The same parallel validity reads exposed a race in tests/evidence.rs. The knowledge fixture's stub `specsync` counts status calls in a file to time its hooks. Two concurrent reads could read the same count, so `archive_summary_apply_fails_closed_when_a_file_changes_during_the_native_status_check` and `archive_summary_restore_keeps_a_file_recreated_during_the_apply` fired their hooks at the wrong time, or not at all. This was also the intermittent verify failure seen during #108. The stub now counts under an atomic `mkdir` lock, so both tests time their hooks exactly. Their assertions are unchanged.

## From the change's testing.md

# Testing

tests/reduce.rs `a_shared_artifact_released_by_an_earlier_removal_does_not_stop_the_trim` gives two superseded runs listed by a cold archive one shared log and their own logs. Both must be trimmed with their own logs, and the shared log kept. It failed before the fix with the same refusal as the repository ("r2: its files changed"). The existing protection-change refusals in tests/evidence.rs keep passing. The trim then runs on this repository at closeout, and `fledge lanes run verify` runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
