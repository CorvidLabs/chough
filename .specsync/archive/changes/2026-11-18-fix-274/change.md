---
id: fix-274
state: archived
type: bug_fix
base_commit: be6c9f126e448bd33945a52261d55db2a9677d3d
---

# Fix 274

## Intent

fix 274

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When a chough or SpecSync command whose JSON examples/feature-bug-loop.sh reads with jq fails or is refused, such as chough bug update --status verified, the example stops at that command with a non-zero exit and that command's error, and runs no later step. No such command's output is piped straight into jq. A run whose commands all succeed passes as before.

## No-spec Rationale

No requirement covers how the example script fails; it is a test harness for the verify lane. Only the script, its test, a case and a changelog fragment change.
