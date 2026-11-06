# Lesson bundle: restore-the-test-closing-brace-lost-when-merging-main-into-know-22-121

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Restore the test closing brace lost when merging main into KNOW-22 (121)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/reduce.rs
- **Acceptance**: tests/reduce.rs compiles again with every test from main and KNOW-22, differing from main only by the KNOW-22 tests and the one doc comment KNOW-22 rewrote; the full suite and fledge lanes run verify pass on Linux.

## Evidence

- Verification commit: `aa73bcfe1943c9ff3d5862123dcc1994c63e335a`
- Base commit: `b3868b844923348903cf67e40dfe6347fb1fb6a1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif chose "Record and merge" for #121 (KNOW-22) on 2026-11-05 at 17:07 MST. He asked for `main` to be merged in after #120 (KNOW-21) landed, with no force push, and for a fresh review if the merge changed the reviewed change.

In merge `e08266c1`, both branches had appended tests to the end of `tests/reduce.rs`, after a shared closing brace. The resolution kept only one brace, so `main`'s last test lost its `}` and the test target stopped compiling. Commit `b3868b84` restores that one brace. That commit changed a meaningful path after the KNOW-22 archive, so this change covers it.

## From the change's testing.md

# Testing

- `cargo fmt --check` passes.
- `cargo test --locked`: 450 passed, 0 failed. That is main's 448, plus the two KNOW-22 tests that are new to it.
- `git diff origin/main -- tests/reduce.rs` removes only the old doc comment of `a_closed_bug_releases_its_loose_fixture`.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
