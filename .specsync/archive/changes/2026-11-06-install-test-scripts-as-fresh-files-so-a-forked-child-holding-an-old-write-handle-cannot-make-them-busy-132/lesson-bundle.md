# Lesson bundle: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Install test scripts as fresh files so a forked child holding an old write handle cannot make them busy (132)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/common/mod.rs, tests/reduce.rs, tests/cli.rs
- **Acceptance**: Test stand-in scripts are installed as fresh files from a child sh with a rename, so a write handle a forked child leaked never makes them busy; the new test fails first with ETXTBSY and passes after; 60 stress rounds at 32 threads show no Text file busy; no assertion, gate, bound or timeout changes; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `74a6133d0015b7652e64762592aa263bc7306987`
- Base commit: `47237d549eb395dfb8c301322d95f371ca4ea3ae`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #132, the fork/fd inheritance family of flaky tests.

## Reproduced

A stress run on Linux aarch64 (Docker) ran `cargo test --test cli --bin chough -- --test-threads=32` 60 times. Two rounds failed, both in `criteria_validate_against_a_hi_family_export_over_16_kib`, with "cannot validate criteria: hi export -- LIFE was unavailable: Unknown: Text file busy (os error 26)". The stderr is kept in `_worktrees/.prof105/fail-132-5.log` and `fail-132-9.log`.

## Cause

A test writes a stand-in script in-process and then runs it. If another test thread forks while the write handle is open, the child holds a copy of that handle until it execs, and Linux refuses to exec a file that is open for writing. SpecSync fixed the same class in CorvidLabs/spec-sync#888.

## Not reproduced

The second instance, `checkpoint::tests::a_held_checkpoint_lock_refuses_as_busy_after_the_wait`, did not fail in those 60 rounds. It stays open in #132 until its stderr is captured.

## From the change's design.md

# Design

## Approach

A shared `install_executable(path, content)` in tests/common pipes the content into `sh -c 'cat > "$1.tmp.$$" && chmod 755 "$1.tmp.$$" && mv "$1.tmp.$$" "$1"'`. The rename gives the path a new inode. Any write handle a fork leaked points at the old inode, and the new one is only ever written inside that `sh` child. `stand_in` uses it. `Fixture::executable` reads the file it is given, which is a read-only handle and cannot block exec, and reinstalls it the same way.

## Risks

A test that relied on an executable keeping its inode would notice; none does.

## Constraints consulted

CorvidLabs/spec-sync#888. The rule against changing assertions or timeouts.

## From the change's testing.md

# Testing

- tests/reduce.rs `a_script_installed_while_an_old_write_handle_is_open_still_runs` covers both `stand_in` and `Fixture::executable`. It fails first, deterministically, with ETXTBSY.
- Stress: the same 60 rounds at `--test-threads=32` must show no "Text file busy".
- No assertion, gate, bound or timeout changes.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
