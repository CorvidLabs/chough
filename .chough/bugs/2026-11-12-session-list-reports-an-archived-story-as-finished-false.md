---
kind: bug
id: 2026-11-12-session-list-reports-an-archived-story-as-finished-false
aliases: []
title: "session list reports an archived story as finished: false"
status: verified
classification: product-defect
scope: in-scope
impact: "The list contradicts itself: archived yet not finished (t38k F12)"
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-session-list-archived-story-is-finished]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-list-archived-story-is-finished
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-list-archived-story-is-finished-77f110
receipts: []
environment: local-macos-arm64
revision: 731aba24c9db412aaa2f1b570043ee4c1f16cc31
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report, F12 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:09:16Z
updated: 2026-11-12T20:09:55Z
---
# session list reports an archived story as finished: false

**What happened:** chough session list gives finished: false for every archived story, at done or closed without done
Actual: finished: false, fix null

**Steps:** Trigger: Archive a session, then run chough session list
1. Close a session with chough session close --reason, or move one into done
2. Run chough session list

**Expected:** An archived story reads finished: true

**Why it matters:** The list contradicts itself: archived yet not finished (t38k F12)

**Fix:** c53da06c (t38b's 626f4c84): session list reports every archived story, at done or closed without done, as finished: true with no fix

## Log

- 2026-11-12T20:09:16Z · Reported (t38k's second fresh-eyes round report, F12 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-list-archived-story-is-finished · by claude-code (hc t382)
- 2026-11-12T20:09:55Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-list-archived-story-is-finished-77f110; fix: c53da06c (t38b's 626f4c84): session list reports every archived story, at done or closed without done, as finished: true with no fix. Red at 731aba24, green at c53da06c · by claude-code (hc t382)
