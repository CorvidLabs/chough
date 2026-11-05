# Lesson bundle: keep-a-checkpoint-reference-whose-record-exists-but-cannot-be-read-and-refuse-the-closed-cycle-drop-review-of-108

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep a checkpoint reference whose record exists but cannot be read, and refuse the closed-cycle drop (review of 108)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, src/stage.rs, tests/reduce.rs
- **Acceptance**: At done, a current-cycle checkpoint reference whose record exists but cannot be read is kept, the step reports refused naming it, no reference is dropped, and done is still recorded; a missing record is still dropped. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `c1eb3073d535efd95245b5e238f42c7d3e906542`
- Base commit: `784b1406d9e5b4b3a53176c3772aaeb003483412`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Reviews of #108 (corvid-agent 5780146055, and Leif's review agent at src/stage.rs:946) found that `evidence::observed_at` turned a read or parse error into None. The closed-cycle drop treats None as "no record" and drops the reference, so a current-cycle record that exists but cannot be read lost its checkpoint reference. Trim and index both refuse that record correctly. REQ-cli-045 allows dropping only a missing record. `observed_at` now separates absence from a read error, and any unreadable referenced record makes the step keep every reference and report `refused` naming it. Done is still recorded.

## From the change's testing.md

# Testing

tests/reduce.rs `an_unreadable_record_keeps_its_reference_and_refuses_the_drop`: a reset 15 days ago, references r2 and u3, and u3's record replaced with unreadable bytes. Done must be recorded, `checkpoint_references` must be refused naming u3, and both references must stay. It fails first. The earlier drop test, where a missing or old record is dropped, keeps passing. `fledge lanes run verify` runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
