---
kind: bug
id: 2026-11-13-the-verify-lane-s-exercise-flakes-when-the-example-s-failing
aliases: []
title: The verify lane's exercise flakes when the example's failing run and its fixed rerun land in the same second
status: open
classification: product-defect
scope: in-scope
impact: Intermittent red CI on unrelated PRs (#244), which costs a rerun and a triage each time
criteria: []
requirements: [REQ-cli-012]
cases: []
evidence:
  - 2026-11-13-fix-247-example-race/2026-11-13-tc-cli-exercise-rerun-waits-past-the-failing-runs-second
receipts: []
environment: ubuntu-latest
revision: e6df71e010f5cea1ce9358fc8c854b22b5e6eeab
found_by: "claude-code (hc t382) (GitHub #247 (t384, from CI run 43357893936 on #244))"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T03:49:04Z
updated: 2026-11-13T06:08:52Z
---
# The verify lane's exercise flakes when the example's failing run and its fixed rerun land in the same second

**What happened:** The Fledge / HI / SpecSync job's verify lane fails at its exercise step: examples/feature-bug-loop.sh's bug update --status verified refuses, saying verified needs a linked passed observation observed after the latest linked failure
Actual: The script reruns at once; with one-second observed times the rerun can share the failure's second, and REQ-cli-012 rightly refuses verified

**Steps:** Trigger: examples/feature-bug-loop.sh on a fast runner, where the failing run of TC-shipping-0001 and the rerun after the fix fall in the same second (CI run 43357893936: 03:19:28.106Z and 03:19:28.962Z)
1. Run examples/feature-bug-loop.sh where the failing run and the rerun land within one second
2. bug update --status verified refuses

**Expected:** The example waits until the clock is past the failing run's recorded second before the rerun, so the bug verifies on any runner; REQ-cli-012's same-second refusal stays as specified

**Why it matters:** Intermittent red CI on unrelated PRs (#244), which costs a rerun and a triage each time

**Fix:** Not fixed yet.

## Log

- 2026-11-13T03:49:04Z · Reported (GitHub #247 (t384, from CI run 43357893936 on #244)) · by claude-code (hc t382)
- 2026-11-13T03:58:27Z · Tracked by the flake window (DRIVE-19); GitHub #247 · by claude-code (hc t382)
- 2026-11-13T06:08:52Z · evidence: 2026-11-13-fix-247-example-race/2026-11-13-tc-cli-exercise-rerun-waits-past-the-failing-runs-second. Red at 190a11ae (t37f's test): with the failing run's recorded second ahead of the clock, the example's rerun-and-verify lines refuse verified with run 43357893936's error · by claude-code (hc t382)
