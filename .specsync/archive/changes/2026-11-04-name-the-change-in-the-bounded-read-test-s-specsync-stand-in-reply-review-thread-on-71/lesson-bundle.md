# Lesson bundle: name-the-change-in-the-bounded-read-test-s-specsync-stand-in-reply-review-thread-on-71

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Name the change in the bounded-read test's SpecSync stand-in reply (review thread on #71)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/cli.rs
- **Acceptance**: native_reads_end_by_their_deadline_and_retain_bounded_output passes with its assertions unchanged, its checkpoint stand-in reply naming bounded-change.

## Evidence

- Verification commit: `dcfc664652de2db94b7f5a4559410f55a1be8e51`
- Base commit: `2d4ba2592011b3be5f5b8f5cc54aad95f3309ed3`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The review-thread fix on #71 (change refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71) makes `checkpoint --native-change` refuse a SpecSync reply that names no change ID. The shared SpecSync stand-in in tests/cli.rs replies `{"next_action":...,"state":"draft"}`, which names none, so `native_reads_end_by_their_deadline_and_retain_bounded_output` failed at its setup checkpoint. Only that test's setup reply changes; no assertion changes.

## From the change's testing.md

# Testing

- `cargo test --locked --test cli native_reads_end_by_their_deadline_and_retain_bounded_output` passes with its assertions unchanged.
- `fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
