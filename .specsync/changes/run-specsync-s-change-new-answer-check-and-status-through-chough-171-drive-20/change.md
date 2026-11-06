---
id: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
state: draft
type: feature
base_commit: 167790489735b3e6a88ec21a20bb2530d4c96138
---

# Run SpecSync's change new, answer, check and status through Chough (171, DRIVE-20)

## Intent

Run SpecSync's change new, answer, check and status through Chough (171, DRIVE-20)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-057 (new): chough change new, answer, check and status run the matching SpecSync step (check with --commit, the others with --json) in the --root directory, pass SpecSync's stdout, stderr and exit code through byte for byte, and write nothing under .chough/. Status keeps the native read bound and, past it, refuses naming a retry that a test runs; check is never stopped by that bound; SIGTERM stops the SpecSync child; a missing specsync refuses naming cargo install --locked specsync@6.2.0 and the step to rerun, which a test runs. Six tests in tests/change.rs fail first because chough change does not exist, and each mutant in testing.md fails one of them. The skill (both copies) and docs name chough change for these four steps. fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
