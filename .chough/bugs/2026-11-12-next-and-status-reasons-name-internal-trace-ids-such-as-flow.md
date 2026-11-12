---
kind: bug
id: 2026-11-12-next-and-status-reasons-name-internal-trace-ids-such-as-flow
aliases: []
title: next and status reasons name internal trace IDs such as FLOW-23
status: open
classification: product-defect
scope: in-scope
impact: Internal IDs in user-facing text (t38k F12)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-and-status-reasons-name-no-trace-id]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-reasons-name-no-trace-id]
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:32Z
updated: 2026-11-12T21:02:32Z
---
# next and status reasons name internal trace IDs such as FLOW-23

**What happened:** The Why line of chough next at plan ends '(FLOW-23)'
Actual: …which only the human gives (FLOW-23).

**Steps:** Trigger: Run chough next at plan with a complete draft waiting for approval
1. Reach plan with a draft native change
2. chough next

**Expected:** Human-facing reasons carry no trace ID

**Why it matters:** Internal IDs in user-facing text (t38k F12)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T21:02:32Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-reasons-name-no-trace-id · by claude-code (hc t382)
