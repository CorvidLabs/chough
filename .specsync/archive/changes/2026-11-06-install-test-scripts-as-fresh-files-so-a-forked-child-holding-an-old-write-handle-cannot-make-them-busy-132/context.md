---
change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
artifact: context
---

# Context

Issue #132, the fork/fd inheritance family of flaky tests.

## Reproduced

A stress run on Linux aarch64 (Docker) ran `cargo test --test cli --bin chough -- --test-threads=32` 60 times. Two rounds failed, both in `criteria_validate_against_a_hi_family_export_over_16_kib`, with "cannot validate criteria: hi export -- LIFE was unavailable: Unknown: Text file busy (os error 26)". The stderr is kept in `_worktrees/.prof105/fail-132-5.log` and `fail-132-9.log`.

## Cause

A test writes a stand-in script in-process and then runs it. If another test thread forks while the write handle is open, the child holds a copy of that handle until it execs, and Linux refuses to exec a file that is open for writing. SpecSync fixed the same class in CorvidLabs/spec-sync#888.

## Not reproduced

The second instance, `checkpoint::tests::a_held_checkpoint_lock_refuses_as_busy_after_the_wait`, did not fail in those 60 rounds. It stays open in #132 until its stderr is captured.
