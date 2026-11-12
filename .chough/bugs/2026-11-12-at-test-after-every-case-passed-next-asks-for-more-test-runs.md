---
kind: bug
id: 2026-11-12-at-test-after-every-case-passed-next-asks-for-more-test-runs
aliases: []
title: At test after every case passed, next asks for more test runs and lists superseded runs as needing attention
status: open
classification: product-defect
scope: in-scope
impact: next stalls a finished test stage (t38m F7)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-at-test-after-every-case-passed-names-stage-advance]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-test-after-every-case-passed-names-stage
receipts: []
environment: local-macos-arm64
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:56:52Z
updated: 2026-11-12T22:56:52Z
---
# At test after every case passed, next asks for more test runs and lists superseded runs as needing attention

**What happened:** chough next at test, every referenced case's latest run passed, printed a run-on 'Continue test: …' action, 'Why: Recorded test evidence is needed before accept', and listed only superseded runs under Observations needing attention, one of them twice
Actual: More test runs asked for; superseded runs listed, one twice

**Steps:** Trigger: At test, run a case red then green, then chough next
1. chough cases run <case> (fails), fix, chough cases run <case> (passes)
2. chough next

**Expected:** chough stage advance to accept; superseded runs counted apart, each observation once

**Why it matters:** next stalls a finished test stage (t38m F7)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T22:56:52Z · Reported (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-test-after-every-case-passed-names-stage · by claude-code (hc t382)
