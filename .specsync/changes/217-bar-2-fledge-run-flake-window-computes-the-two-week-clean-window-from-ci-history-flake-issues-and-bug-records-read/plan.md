---
change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
artifact: plan
---

# Plan

## Approach

Add a read-only Python script and its fixtures, two fledge tasks, one CI step and the
docs. Chough's binary, skill and records are unchanged.

## Out of scope

- The `flake` label, labelling #187 and #132, and the bug record notes: the seed PR.
- The daily scheduled CI run (REQ-cli-064): its own PR.
- Recording a window receipt: that happens at release.

## Steps

1. Commit the failing fixtures, `.github/scripts/test-flake-window.py`, and record
   the red run.
2. Add `.github/scripts/flake-window.py`, the `flake-window` and
   `ci-flake-window-fixtures` fledge tasks, and the fixtures step in the
   `CI gate fixtures` job.
3. Docs: a "Flakes and the clean window" section in docs/ci.md, a release step in
   docs/releasing.md, a pointer in docs/test-plan-1.0.md, and a CHANGELOG line under
   Unreleased.
4. Record green, run the full local suite, run the audit, then review.

## Risks

- GitHub API limits: about 60 to 100 calls per run, well under the hourly limit.
- Time zones: every day boundary is UTC.

## Constraints consulted

specs/cli/cli.3md says hosted runners have no Git identity, so the fixtures never
commit. FLOW-10 and FLOW-16: no human decision is recorded. LIFE-19: the release
keeps a short receipt, and the raw input stays out of Git.
