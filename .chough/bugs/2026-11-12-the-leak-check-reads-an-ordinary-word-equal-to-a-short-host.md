---
kind: bug
id: 2026-11-12-the-leak-check-reads-an-ordinary-word-equal-to-a-short-host
aliases: []
title: The leak check reads an ordinary word equal to a short host name as a host-name leak
status: deferred
classification: product-defect
scope: out-of-scope
impact: False leak refusals on some machines (found while running the fourth round)
criteria: []
requirements: [REQ-records-006]
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
created: 2026-11-12T23:36:10Z
updated: 2026-11-12T23:36:12Z
---
# The leak check reads an ordinary word equal to a short host name as a host-name leak

**What happened:** chough doctor and every committed-record write treat any word equal to the local host name, case-insensitively and with no length floor beyond three letters, as a host-name leak: on a machine whose short host name is a common word, a lesson using that word was flagged, and a relayed --where quoting a human's words with it was refused
Actual: refused:leak with kind host-name, or a doctor leak finding

**Steps:** Trigger: On a machine whose short host name is a common word, write that word into a committed record
1. Use the word in a lesson or a relayed --where
2. chough doctor, or the write itself

**Expected:** A common word not read as this machine's name, or a rule the person can see and adjust

**Why it matters:** False leak refusals on some machines (found while running the fourth round)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:36:10Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:12Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
