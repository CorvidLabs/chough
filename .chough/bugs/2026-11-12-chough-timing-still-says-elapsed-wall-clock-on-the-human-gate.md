---
kind: bug
id: 2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate
aliases: []
title: chough timing still says elapsed wall-clock on the human-gate line, and its notes say read read-only
status: verified
classification: product-defect
scope: out-of-scope
impact: Report polish (t38n F5)
criteria: []
requirements: [REQ-reporting-001]
cases: []
evidence:
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:01Z
updated: 2026-11-13T03:01:28Z
---
# chough timing still says elapsed wall-clock on the human-gate line, and its notes say read read-only

**What happened:** chough timing prints 'waiting 9s (elapsed wall-clock)' on the human-gate line, though the CHANGELOG says the phrase appears once, and a note says 'native record files read read-only'
Actual: The phrase on the gate line; 'read read-only'

**Steps:** Trigger: Run chough timing on a session with a closed human gate
1. chough timing

**Expected:** elapsed wall-clock said once; no doubled word

**Why it matters:** Report polish (t38n F5)

**Fix:** bc275212 (t37f's 91cf8d50): timing says elapsed wall-clock once, in its header, on no gate, run or fix line, and its note reads read-only once (REQ-reporting-001)

## Log

- 2026-11-12T23:36:01Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:03Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T03:00:46Z · status: deferred -> open; evidence: 2026-11-13-cli-output-fixes/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line. Taken up in 0.6.1 PR 6 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at 1a57d6f3 · by claude-code (hc t382)
- 2026-11-13T03:01:28Z · status: open -> verified; evidence: 2026-11-13-cli-output-fixes/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376; fix: bc275212 (t37f's 91cf8d50): timing says elapsed wall-clock once, in its header, on no gate, run or fix line, and its note reads read-only once (REQ-reporting-001). Red at 1a57d6f3, green at bc275212 · by claude-code (hc t382)
