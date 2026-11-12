---
kind: bug
id: 2026-11-12-at-accept-with-the-change-approved-next-shows-a-refused-gate
aliases: []
title: At accept with the change approved, next shows a refused gate instead of the review preparation
status: verified
classification: product-defect
scope: in-scope
impact: The accept stage reads as blocked (t38k F4)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-at-accept-while-approved-names-the-review-preparation]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-while-approved-names-the-review
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-while-approved-names-the-review-88be5c
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:46Z
updated: 2026-11-12T21:04:01Z
---
# At accept with the change approved, next shows a refused gate instead of the review preparation

**What happened:** chough next at accept with native state approved gives 'Gate not open (refused:native-approved)' and a raw specsync change status as the action
Actual: Gate not open (refused:native-approved)

**Steps:** Trigger: Reach accept while the native change is approved, before its check
1. Advance to accept with the change approved
2. chough next

**Expected:** The accept step's preparation in order: change check --no-commit when the deltas add requirements, the cases, the review gate, the commit, change check

**Why it matters:** The accept stage reads as blocked (t38k F4)

**Fix:** 1f37428f (t37f's 3e5a0cc5): at accept while approved, next names the review preparation in order (REQ-cli-021)

## Log

- 2026-11-12T21:02:46Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-while-approved-names-the-review · by claude-code (hc t382)
- 2026-11-12T21:02:46Z · Added despite similar reports: The verified bugs cover accept while verifying (the commit-first ship) and #76's stage guidance; this one is accept while the change is still approved, before its check, where next shows a refused gate · by claude-code (hc t382)
- 2026-11-12T21:04:01Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-while-approved-names-the-review-88be5c; fix: 1f37428f (t37f's 3e5a0cc5): at accept while approved, next names the review preparation in order (REQ-cli-021). Red at 5195bd67, green at cca98758 · by claude-code (hc t382)
