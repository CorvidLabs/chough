# Lesson bundle: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Show a failed native command as exit N or killed by signal N instead of Rust debug text (issue 87)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, tests/cli.rs, tests/native_failure.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: A native command that exits 2 is shown as exit 2 in status and next and never as Some. A native command ended by SIGTERM is shown as killed by signal 15. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `952e6dc945c89031a230bae34ebab9c028ee4f7c`
- Base commit: `91a1fdd82a4e314e5037c6256f17199ef0d604f0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/87 (field test 2026-11-03, chough
a33c747): a failed native read is shown as `specsync change status does-not-exist --json
(failed): exit=Some(1)`, Rust's debug form of an optional exit code. Expected: plain text
such as `exit 1` or `killed by signal N`. The unknown ID in the issue's repro is #63,
fixed by #71; the text is still produced on main 91a1fdd by `failure` in src/records.rs
for any failed native command. tests/cli.rs asserted the debug form (`exit=Some(2)`);
that expectation is corrected with this fix.

Criteria: LIFE-7. Authority: Leif's 2026-11-04 standing approval for bounded Chough bug
fixes and his brief to fix issues 81 to 88 one PR each.

## From the change's design.md

# Design

- src/records.rs: `Captured` gains `signal: Option<i32>` (Unix: the terminating signal
  from `ExitStatusExt::signal`; None elsewhere or when it exited or timed out).
  `failure` starts with `exit <code>`, `killed by signal <number>`, `timed out` or
  `no exit status`, then the bounded stdout and stderr as before.
- No other output changes; the native section state stays `failed`.

## From the change's testing.md

# Testing

- tests/native_failure.rs:
  - `a_failed_native_read_shows_its_exit_in_plain_words`: a stub `specsync` that exits 2
    gives `status` content starting `exit 2` and a `next` line with `(failed): exit 2`,
    never `Some(`.
  - `a_native_read_killed_by_a_signal_says_so`: a stub that kills itself with SIGTERM
    gives `killed by signal 15`.
- tests/cli.rs: the stderr-flood case now expects `exit 2` instead of `exit=Some(2)`.
- Each fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
