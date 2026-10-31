# Lesson bundle — bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Bound external commands in time and memory and honor validation stop requests after waiting
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/cases.rs, src/records.rs, src/validate.rs, src/main.rs, Cargo.toml, Cargo.lock, tests/, .chough/, docs/cli.md
- **Acceptance**: Every external command Chough runs (case bindings, native status and other native reads) is read through one bounded runner: each stream retains at most its cap while reading and keeps draining beyond it, both streams are read concurrently, and the run ends by its deadline. On a deadline, or when output is still held open after the command exits, the command and its descendants are stopped (Unix: its process group; Windows: its process tree), and any reader still blocked is abandoned rather than awaited (BUG-0010, BUG-0011, REQ-cli-011, REQ-cli-006).
- **Acceptance**: A case run with a 1 s deadline whose descendant holds the pipes returns within a few seconds as failed with a timeout limitation; a command that exits while a descendant holds the pipes returns by its deadline with a limitation naming the stopped descendants; output beyond 1 MiB per stream is counted, not retained, and the artifact notes the discarded byte count (BUG-0010).
- **Acceptance**: Native reads keep the 16 KiB display bound and its truncation note, parse complete stdout up to 4 MiB, retain at most 16 KiB of stderr, and report a response over the bound, a failure, an undecodable or unavailable command, or a deadline (default 120 s, CHOUGH_NATIVE_TIMEOUT_SECONDS 1 to 3600) explicitly; gated moves refuse each of them as native-unknown, and status and next show the state and next action (BUG-0011, REQ-cli-003).
- **Acceptance**: A repeated validation run checks for a stop request and a paused checkpoint immediately before every attempt, including after its interval, and waits in slices so a stop during a long interval ends the run promptly with no further attempt (BUG-0009, REQ-cli-011).
- **Acceptance**: Existing behavior these rules do not name is unchanged; the full suite, the feature-and-bug exercise and native structural checks pass on Ubuntu, macOS and Windows CI; regression tests cover descendants, deadlines and output bounds per platform and the stop-during-interval case, and BUG-0009 to BUG-0011 are verified by passing observations.

## Evidence

- Verification commit: `490190506956d7fa801d999a99ad6f21be2410fb`
- Base commit: `31d5105343a46728551648375bf9a33e213a3ce3`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Three execution defects were reproduced on main `f19823d` (macOS arm64) in disposable
copies of `examples/shipping`, each recorded with a failed observation and a log under
`.chough/artifacts/`:

- BUG-0009: `validate.rs` checks the stop file before sleeping, not after; a stop 1.5 s
  into a 4 s interval still allowed a second attempt.
- BUG-0010: `cases.rs` reads each pipe with `read_to_end` and truncates afterwards, and
  on timeout kills only the direct child, then joins readers that wait for EOF. A 1 s
  deadline returned after 20 s and a 3 s deadline after 21 s (recorded passed) while a
  `sleep` descendant held the pipes; 300 MB of output peaked at 578 MiB.
- BUG-0011: `records.rs` native reads use `Command::output()`: unlimited stdout and
  stderr, no deadline. 400 MB from a stand-in peaked at 774 MiB before `oversized`; a
  stand-in that never exits kept `chough next` waiting past 30 s.

BUG-0004's archived fix (parse complete output up to 4 MiB, display 16 KiB with a
note, reject oversized and undecodable output) stays as it is; this change bounds the
capture that feeds it. Gate rules are a separate change (BUG-0006 to BUG-0008).

## From the change's design.md

# Design

`records.rs` gains one bounded runner used by case execution and every native read:

```text
run_bounded(root, argv, Limits { stdout_cap, stderr_cap, deadline }) -> Captured {
    exit_code, timed_out, stdout, stdout_total, stderr, stderr_total,
    descendants_stopped, capture_incomplete }
```

- One reader thread per stream retains up to its cap, counts every byte and reports EOF
  on a channel. The caller polls the child every 25 ms.
- Deadline reached: stop the tree (Unix group SIGKILL; Windows `taskkill /T /F`), reap
  the child, wait up to 1 s for readers, then abandon them.
- Child exited but a stream is still open after a 2 s grace (bounded by the deadline):
  stop the tree, wait up to 1 s, then abandon; `descendants_stopped` is set.
- Caps: cases 1 MiB per stream (artifact keeps the existing 1 MiB total and adds
  `[truncated: <stream> wrote N bytes; M discarded]`); native stdout 4 MiB + 1 (so
  "over the bound" is exact), native stderr 16 KiB.
- Native deadline 120 s, overridable by `CHOUGH_NATIVE_TIMEOUT_SECONDS` (1 to 3600) for
  slow repositories and tests. A native deadline gives section state `timed-out`;
  `native_json` returns no value, so gates refuse as native-unknown.
- Case limitations add `timed out after N s; the command and its descendants were
  stopped`, `descendants still held output after exit and were stopped`, and `output
  capture incomplete`. A run whose capture is incomplete cannot be `passed`.
- Validation: before every attempt, and after waiting, stop if the stop file exists
  (`stopped`) or the checkpoint stage is paused (`paused`, resumable). The interval is
  slept in 200 ms slices that re-check both.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-011 | `tests/cases.rs`: deadline with a pipe-holding descendant; exit with a pipe-holding descendant; per-stream output bound; `tests/optional.rs`: stop during the interval and pause before an attempt |
| REQ-cli-003 | `tests/cli.rs`: native stand-in that hangs (deadline override), that writes beyond the parse bound, and that floods stderr |
| REQ-cli-006 | unit tests of the bounded reader: retained bytes never exceed the cap while totals keep counting |

Unix tests use `sh` descendants; Windows tests use `cmd /c start /b` descendants; both
run in hosted CI. Timing assertions use generous margins (deadline + 10 s) to avoid
flakiness and still fail the 20 s reproductions. Regression: full `cargo test`,
`fledge lanes run verify`, and a rerun of the recorded reproductions on macOS.

## Where these lessons go

- `specs/cli/context.md`
