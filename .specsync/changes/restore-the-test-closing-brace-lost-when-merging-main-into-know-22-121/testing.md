---
change: restore-the-test-closing-brace-lost-when-merging-main-into-know-22-121
artifact: testing
---

# Testing

- `cargo fmt --check` passes.
- `cargo test --locked`: 450 passed, 0 failed. That is main's 448, plus the two KNOW-22 tests that are new to it.
- `git diff origin/main -- tests/reduce.rs` removes only the old doc comment of `a_closed_bug_releases_its_loose_fixture`.
- `fledge lanes run verify` passes on Linux.
