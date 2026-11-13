---
id: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
state: approved
type: bug_fix
base_commit: 9d53604071df50b537adccad0ccf5cac9a2fc32b
---

# 0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json

## Intent

0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough change ship prints SpecSync output, then a short text summary of the review, the ship, the gate close and the move into done, at most 12 lines; with --json it prints every step full report as before (red then green)

## No-spec Rationale

Not applicable
