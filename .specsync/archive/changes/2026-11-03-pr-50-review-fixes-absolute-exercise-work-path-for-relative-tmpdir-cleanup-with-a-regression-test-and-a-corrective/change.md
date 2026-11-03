---
id: pr-50-review-fixes-absolute-exercise-work-path-for-relative-tmpdir-cleanup-with-a-regression-test-and-a-corrective
state: archived
type: bug_fix
base_commit: 250d6378614e9872ee0d55d493f4c2a7a920c87a
---

# PR 50 review fixes: absolute exercise work path for relative TMPDIR cleanup with a regression test, and a corrective agent-assisted observation

## Intent

PR 50 review fixes: absolute exercise work path for relative TMPDIR cleanup with a regression test, and a corrective agent-assisted observation

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With a relative TMPDIR (for example TMPDIR=.), a directory the exercise creates is made absolute before any cd, so a passing full run removes it
- A passed-in directory, a prepare-cold-start directory and a failed run's directory are still kept
- A focused regression test runs the script's exact setup and cleanup source with TMPDIR=. and proves both
- OBS-human-observation-174caaa1021cfdf5a553f7a3 stays byte-identical, and a corrective agent-assisted observation states the agent command check without claiming a human witness
- fledge lanes run verify passes

## No-spec Rationale

Example script and evidence-record fixes; no requirement or criterion text changes
