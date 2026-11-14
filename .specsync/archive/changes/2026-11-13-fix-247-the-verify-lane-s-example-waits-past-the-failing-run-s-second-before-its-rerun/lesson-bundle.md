# Lesson bundle: fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #247: the verify lane's example waits past the failing run's second before its rerun
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: examples/feature-bug-loop.sh, tests/cli.rs, docs/exercises/feature-bug-loop.md, CHANGELOG.md
- **Acceptance**: examples/feature-bug-loop.sh waits until the clock is past the failing run's recorded second before its fixed rerun, so bug update --status verified succeeds even when the failing run and the rerun would land in the same second; REQ-cli-012's same-second refusal is unchanged (red then green)

## Evidence

- Verification commit: `96817a4a3de55d4fd7f6ef5f0a7e567f12d9d1e7`
- Base commit: `e6df71e010f5cea1ce9358fc8c854b22b5e6eeab`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On PR #244's head f904d498, CI run 43357893936 failed the verify lane's `exercise`
step: `examples/feature-bug-loop.sh` logged the failing run of `TC-shipping-0001` at
03:19:28.106Z and the fixed rerun at 03:19:28.962Z. Observed times have one-second
resolution, and REQ-cli-012 refuses `verified` for a pass observed in the same second
as the latest linked failure, so `chough bug update --status verified` refused, as
specified. A fast runner reaches the rerun in under a second. t384 filed it as GitHub
#247; #169 was a similar second-boundary flake.

On 2026-11-13T03:44Z Leif approved the fix ("Approve (Recommended)", relayed by t370):
the script waits past the failing run's second before rerunning; t37f builds it with
a failing test first. REQ-cli-012's same-second rule is specified and stays as it is;
weakening it was ruled out.

## From the change's testing.md

# Testing

A test in `tests/cli.rs` runs the example's rerun-and-verify lines, cut out between
comment markers as the existing exercise tests do, against a small fixture whose
failing run's recorded second is ahead of the clock, so the rerun lands at or before
it. Red at the test commit: `bug update --status verified` refuses with run
43357893936's error. Green at the fix: the script waits past that second, and the bug
verifies. The full local suite at the final head, and the verify lane's exercise in CI.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | `tests/cli.rs`: the example's rerun lands after the failing run's second, and the same-second refusal itself is unchanged |

## Where these lessons go

- `specs/cli/cli.3md`
