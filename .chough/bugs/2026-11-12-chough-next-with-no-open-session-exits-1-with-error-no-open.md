---
kind: bug
id: 2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open
aliases: []
title: "chough next with no open session exits 1 with Error: no open session"
status: verified
classification: product-defect
scope: out-of-scope
impact: First-run polish (t38n F10)
criteria: []
requirements: [REQ-cli-021]
cases: []
evidence:
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838
  - 2026-11-13-cli-output-fixes/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:26Z
updated: 2026-11-13T03:01:28Z
---
# chough next with no open session exits 1 with Error: no open session

**What happened:** After the move into done, the normal state, chough next prints 'Error: no open session' and exits 1
Actual: An error and exit 1

**Steps:** Trigger: Run chough next with no open session
1. Archive the session at done
2. chough next

**Expected:** A plain message naming chough session new <key>, exit 0

**Why it matters:** First-run polish (t38n F10)

**Fix:** bc275212 (t37f's 91cf8d50): with no open session, next says so, names chough session new <key> and exits 0; status still refuses (REQ-cli-021, REQ-history-006)

## Log

- 2026-11-12T23:36:26Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:26Z · Added despite similar reports: The verified bug is a session kept open at done being told to start a new one; this one is the normal state after done, no open session, reported as an error with exit 1 · by claude-code (hc t382)
- 2026-11-12T23:36:27Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T03:00:46Z · status: deferred -> open; evidence: 2026-11-13-cli-output-fixes/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0, 2026-11-13-cli-output-fixes/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses. Taken up in 0.6.1 PR 6 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at 1a57d6f3 · by claude-code (hc t382)
- 2026-11-13T03:01:28Z · status: open -> verified; evidence: 2026-11-13-cli-output-fixes/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838, 2026-11-13-cli-output-fixes/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968; fix: bc275212 (t37f's 91cf8d50): with no open session, next says so, names chough session new <key> and exits 0; status still refuses (REQ-cli-021, REQ-history-006). Red at 1a57d6f3, green at bc275212 · by claude-code (hc t382)
