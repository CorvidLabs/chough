# Lesson bundle: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Give real assertions to the nine criteria the audit found with no selected assertion (138)
- **Kind**: BugFix
- **Specs**: cli, reporting
- **Paths**: tests/cli.rs, tests/stages.rs, tests/bugs.rs, tests/workflow_authority.rs, tests/cases.rs, tests/reduce.rs, tests/timing.rs, examples/measure-report.sh, docs/cli.md, CHANGELOG.md
- **Acceptance**: Each of the nine criteria the audit found with no selected assertion (#138) has a test that asserts it: REQ-cli-002 lesson labels and no lesson store; REQ-cli-007 no second checkpoint store and no APS; REQ-cli-012 the upgrade note for new-form bug IDs; REQ-cli-029 recorded authority before approval and no human decision from labels, and no skipped cold-start proof promoted to a pass; REQ-cli-031 the walkthrough opening; REQ-cli-045 librarian off and no reset time; REQ-reporting-003 before/after measurement through examples/measure-report.sh. Each new assertion fails under a recorded targeted mutation and passes restored; no criterion, test, bound or timeout is weakened. The measure-report test fails first, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `7894feaa3aeacdbdc1b421ed3357e2a8702280d5`
- Base commit: `260ecc949ea01214f4bec43b30ea4226ce8fae59`
- Verified by: `specsync check --spec cli --spec reporting`

## From the change's context.md

# Context

Issue #138, from the audit's Chough assertion review (main `700dad96`). It mapped 233 acceptance criteria to passing assertions; nine had none. The audit's `REQ-…/AC-N` labels are ordinals, not product IDs. Two of the nine criteria changed after the audit: KNOW-21 (#120) rewrote REQ-cli-045, so its no-reset criterion now reads "no observation reference is dropped".

| Audit label | What is not asserted today |
|---|---|
| REQ-cli-002/AC-3 | The skill's three certainty labels, source and limits, and that a project decision outranks a lesson. Also that Chough keeps no lesson store, index or ledger: the knowledge notes only point to lessons. |
| REQ-cli-007/AC-4 | No second writable checkpoint store, and no adapter such as APS required. |
| REQ-cli-012/AC-13 | The CLI documentation's upgrade note for new-form bug IDs. |
| REQ-cli-029/AC-2 | Authority recorded with source, scope and limits before approval; asking when it is missing or exceeded; agent labels and bot approvals never becoming human decisions. |
| REQ-cli-029/AC-5 | Source/package verification kept apart from a fresh-agent cold start; no skipped cold-start proof promoted to a pass. |
| REQ-cli-031/AC-3 | The walkthrough begins with the new-project setup path. |
| REQ-cli-045 (audit AC-2) | With the librarian off, the references are unchanged and reported `skipped`. |
| REQ-cli-045 (audit AC-3) | With no reset time, no observation reference is dropped. |
| REQ-reporting-003 (audit AC-4) | Before/after measurement: one input root, explicit binary identities, local work apart from native wait, incomplete reads kept partial or unknown, and native slowness never reported fixed by a local change. |

## Leif's decisions

- The 95% plan, item C6: these nine get real assertions, in one issue, one change and one PR.
- 2026-11-05 21:52 MST, through the orc, "Both recommended":
  - REQ-cli-029/AC-5 is asserted in Chough's evidence model plus ROADMAP text.
  - REQ-reporting-003's measurement criterion is asserted through a new script, `examples/measure-report.sh`, plus a test.

No runtime behavior changes, and no new REQ or HI IDs are taken.

## From the change's design.md

# Design

## Tests

New tests go into the existing test file of each requirement; no new test file is added, so no spec's file list changes. Every test that reads repository text normalizes CRLF and checks a CRLF copy too (#131 lesson). For each new assertion, one targeted mutation is applied by hand, the test is shown to fail, and the file is restored. The logs are kept as Chough evidence.

## examples/measure-report.sh

`sh examples/measure-report.sh OLD NEW ROOT [RUNS]` (POSIX sh, like the exercise script; RUNS defaults to 3).

- Refusals exit 2 with `usage: sh examples/measure-report.sh OLD NEW ROOT [RUNS]`: a missing or non-executable binary, a ROOT that is not a Git project with a commit, or a RUNS that is not a positive integer.
- It prints, in order:
  - `root <absolute path> revision <commit> dirty <yes|no>`
  - `native_timeout_seconds <CHOUGH_NATIVE_TIMEOUT_SECONDS or default>`
  - one `binary <old|new> sha256 <hex> version <--version output>` line per binary
  - one `run <side> <n> <first-process|repeated> exit=<code> wall_ms=<n> native_ms=<n> local_ms=<n> native_reads=<n> incomplete_reads=<n>` line per run of `CHOUGH_TRACE=1 <binary> --root ROOT next --json`. `native_ms` sums the trace's completed (`observed`) native reads, `local_ms` is wall minus that, and any other native state counts as incomplete.
- Summary:
  - `summary local old_median_ms=… new_median_ms=…`, always printed.
  - `summary native old_median_ms=… new_median_ms=…` only when every native read completed.
  - Otherwise `summary native incomplete: <k> native reads timed out or were unavailable; no native comparison`.
- No line combines local and native time into one verdict, so a local change is never reported as fixing native slowness.
- docs/cli.md's profiling procedure names the script.

## From the change's testing.md

# Testing

Each task names its test. The failing test first is `measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart`: it fails until examples/measure-report.sh exists. The other new tests assert behavior and text that exist today, so each is shown real by one targeted mutation that makes it fail, then restored:

| Requirement | Mutation that must fail the new test |
|---|---|
| REQ-cli-002 | Drop "always outranks a lesson" from accept.md; copy the lesson sentence into the generated module note. |
| REQ-cli-007 | Have `checkpoint` also write a second checkpoint copy. |
| REQ-cli-012 | Drop the upgrade sentence from docs/cli.md. |
| REQ-cli-029 authority | Move plan.md's authority sentence after the approve example. |
| REQ-cli-029 cold start | Let `cases record` accept a missing verdict; claim a passed cold start in ROADMAP. |
| REQ-cli-031 | Move "## New project" after "## Install". |
| REQ-cli-045 | Drop the librarian-off skip; drop observations when no reset time is read. |
| REQ-reporting-003 | In the script, count a timed-out read as completed. |

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-002 | tests/cli.rs (new lesson-source tests) |
| REQ-cli-007 | tests/stages.rs (new single-store test) |
| REQ-cli-012 | tests/bugs.rs (new upgrade-note test) |
| REQ-cli-029 | tests/workflow_authority.rs, tests/cases.rs (new authority and cold-start tests) |
| REQ-cli-031 | tests/cli.rs (new walkthrough-opening test) |
| REQ-cli-045 | tests/reduce.rs (new librarian-off and no-reset tests) |
| REQ-reporting-003 | tests/timing.rs `measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
- `specs/reporting/context.md`
