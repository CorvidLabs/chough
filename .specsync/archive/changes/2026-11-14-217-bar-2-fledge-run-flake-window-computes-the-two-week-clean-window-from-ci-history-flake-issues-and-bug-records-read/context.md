---
change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
artifact: context
---

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
