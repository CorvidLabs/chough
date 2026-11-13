---
kind: bug
id: 2026-11-12-chough-change-ship-prints-specsync-s-text-a-long-json
aliases: []
title: chough change ship prints SpecSync's text, a long JSON checkpoint and the move summary together
status: deferred
classification: product-defect
scope: out-of-scope
impact: Output polish (t38n F6)
criteria: []
requirements: [REQ-cli-059]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:03Z
updated: 2026-11-12T23:36:05Z
---
# chough change ship prints SpecSync's text, a long JSON checkpoint and the move summary together

**What happened:** chough change ship interleaves SpecSync's output, about 100 lines of JSON checkpoint and relayed decision, then the move summary
Actual: Mixed text and a long JSON document

**Steps:** Trigger: Ship a change with chough change ship <id> --close-gate
1. chough change ship <id> --close-gate <gate> with the relay

**Expected:** A short text summary by default, the JSON under --json, as for stage moves

**Why it matters:** Output polish (t38n F6)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:36:03Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:05Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
