# Lesson bundle: fix-262-open-race

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 262 open race
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/records.rs, src/sessions.rs, tests/special_files.rs, .chough/cases/history.non-regular-files-open.cases.toml, changelog.d/2026-11-18-fix-262-open-race.md
- **Acceptance**: The bounded record reader opens a path without blocking and checks on the opened handle that it is a regular file, so a FIFO at the path at the moment of the open is refused at once; chough session shelve reads session.md and each listed file through the same non-blocking open.

## Evidence

- Verification commit: `bb65c09d060400389c31ab2218b20c6c16f7ad92`
- Base commit: `caa6ee65a46ec3f4798e0a64c22bb263d062814a`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

The agent review of #305 (fix for #262) found a window the fix leaves open. The bounded record reader checks that a path is a regular file and then opens it by path, as two separate steps. If the file is replaced with a FIFO between them, the check passes and the open still blocks for ever. The reviewer reproduced it by replacing `session.md` after the check: `session list` ran past a three second deadline. `session shelve` has the same shape for `session.md` and for the files it lists.

fix-262 is already reviewed and archived, so this is its own change on the same branch and pull request. Leif chose on 2026-11-17 to fix this before #305 merges. No requirement text changes.

## From the change's design.md

# Design

One function, `open_regular` in `src/records.rs`, opens a path for reading and returns it only when the opened handle is a regular file.

- On Unix the open carries `O_NONBLOCK`, so opening a FIFO returns at once instead of waiting for a writer. The type is then read from the opened handle with `fstat`, not from the path, so there is no gap between a check and the open for a replacement to slip into. `O_NONBLOCK` changes nothing for reading a regular file.
- On other platforms there are no FIFOs to block on; the path check stays before the open.
- `read_bounded_record` reads through `open_regular` and drops its earlier path check.
- `session shelve` reads `session.md` and each file it lists through `open_regular` too.

Rejected: keeping the path check and adding a second check after the open. A blocking open of a FIFO never returns, so a check after it never runs.

Risk: `libc::O_NONBLOCK` is used directly; `libc` is already a Unix dependency of this crate.

## From the change's testing.md

# Testing

`cargo test --locked --bin chough records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking` (Unix only) calls `open_regular` and `read_bounded_record` on a FIFO from a thread with a 10 second limit and checks both refuse it as not a regular file, and that a regular file still reads. The reader no longer has a check before its open, so a FIFO present at the open is the replaced-after-the-check case. This test was not seen failing first: it passes on the previous code too, where the earlier path check catches a FIFO that is already there, and the reviewer's replacement between check and open needs an interposer this suite does not have. The two `tests/special_files.rs` tests from fix-262 still pass and cover the commands end to end. Case `TC-history-a-fifo-at-the-open-is-refused-without-blocking` binds the unit test.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
