---
id: fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun
state: implementing
type: bug_fix
base_commit: e6df71e010f5cea1ce9358fc8c854b22b5e6eeab
---

# Fix #247: the verify lane's example waits past the failing run's second before its rerun

## Intent

Fix #247: the verify lane's example waits past the failing run's second before its rerun

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- examples/feature-bug-loop.sh waits until the clock is past the failing run's recorded second before its fixed rerun, so bug update --status verified succeeds even when the failing run and the rerun would land in the same second; REQ-cli-012's same-second refusal is unchanged (red then green)

## No-spec Rationale

A fix within the existing requirements; no contract text changes: REQ-cli-012's same-second rule stays as written.
