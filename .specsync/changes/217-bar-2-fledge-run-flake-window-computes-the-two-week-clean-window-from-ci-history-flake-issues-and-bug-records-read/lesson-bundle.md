# Lesson bundle: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: #217 bar 2: fledge run flake-window computes the two-week clean window from CI history, flake issues and bug records, read-only, as a receipt with a clean, running or blocked verdict
- **Kind**: Feature
- **Specs**: cli
- **Paths**: .github/scripts/flake-window.py, .github/scripts/test-flake-window.py, .github/workflows/ci.yml, fledge.toml, docs/ci.md, docs/releasing.md, docs/test-plan-1.0.md, CHANGELOG.md
- **Acceptance**: fledge run flake-window prints one JSON receipt and writes nothing: clean after 14 days of green main CI on macOS and Linux with no rerun and no open flake; running, naming the latest restart and when it can first be clean, after a red or skipped main run, a rerun on any branch, a filed flake or a day without a green main run on either OS; blocked while a flake issue or DRIVE-19 bug is open, a closed flake has no green main run since, or a flake issue has no bug record; exit 2 and never clean when GitHub cannot be read in full (red then green in .github/scripts/test-flake-window.py)

## Evidence

- Verification commit: `3be6d7da830fbba16b8ad741c5dfe5b6d2d7db89`
- Base commit: `4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #217 is Chough 1.0's bar item 2 (LIFE-21): "CI is green with zero known flaky
tests on macOS and Linux for two consecutive weeks". docs/test-plan-1.0.md (bar 2)
already says where flakes live: a GitHub `flake` label plus one bug record per flake
in `.chough/bugs/`. It also says the window "is computed from the CI run history, not
from memory". Nothing computed it yet, and the label did not exist.

On 2026-11-13 ch10-flake (claude-code, hc t38r) sent a design with four questions to
the coordinator (t370), who relayed Leif's answers at 03:30Z. Two of them bind this
change:
- Q1, where the check lives: "Repo script + fledge task (Recommended)". Rejected: a
  `chough` command, which would add a GitHub dependency to Chough, and a daily
  GitHub job that posts the result to an issue.
- Q2, what restarts the window: "Any red, rerun or new flake (Recommended)".
  Rejected: counting only failures traced to a filed flake.

Q3, a daily CI run, is a separate PR (REQ-cli-064). Seeding the label and linking
the existing flake bug records is also a separate PR. Requirement IDs come from
t382, who owns the records: REQ-cli-063 stays unused because closed PR #233 used it.

Facts measured on 2026-11-13:
- The CI workflow runs about 100 times a day across all branches.
- GitHub returns at most 1,000 runs for a filtered query, so the check reads one UTC
  day at a time.
- `run_attempt` above 1 marks a rerun. One PR run was rerun at 03:33Z that day.
- `bug update` has no criterion flag, so a flake record names DRIVE-19 and its
  issue (`GitHub #<n>`) in a Log note.

## From the change's design.md

# Design

`.github/scripts/flake-window.py` uses only Python's standard library, like
`check-specsync-archive.py`, and works in three parts:
- **fetch:** `gh api` reads the CI workflow's runs one UTC day at a time over a
  45-day lookback, because a run can be rerun for up to 30 days. It also reads the
  jobs of every main run created within the last 15 days, plus main runs after each
  closed flake until a green one is found, and every issue labelled `flake`, with
  pull requests left out.
- **read:** the front matter and text of every `.chough/bugs/*.md` record.
- **compute:** a pure function that turns those inputs and `now` into the receipt.

`--input <file>` replaces the fetch with saved input, and `--save-input <file>`
keeps the fetched input, so a receipt can be recomputed. The receipt carries the
input's SHA-256.

A main run is one on branch main whose event is push, schedule or workflow_dispatch.
A run is green for an OS when its `Product` job for that OS succeeded. A main run
whose conclusion is not success, or that has a job that did not succeed, is a red
main run. The DRIVE-19 restart and block read every bug record that names DRIVE-19
in its front matter or text. A record counts as closed when its status is verified,
duplicate, wont-fix or fixed-upstream (`src/bugs.rs` CLOSED). Exit codes: 0 with a
receipt, 1 under `--require-clean` unless the verdict is clean, 2 when GitHub's data
is unavailable or incomplete.

## From the change's testing.md

# Testing

`.github/scripts/test-flake-window.py` (unittest, standard library only) builds run,
job, issue and bug fixtures in code and checks the receipt. It also runs the script
with a fake `gh` on PATH, so the fetch, pagination and fail-closed exits are
exercised without the network. Red at the test commit, green at the fix. Chough cases
bind each test class through the `command` adapter.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-062 | `.github/scripts/test-flake-window.py`: CleanWindowTests, RestartTests, BlockedTests, FailClosedTests and WiringTests |

## Where these lessons go

- `specs/cli/cli.3md`
