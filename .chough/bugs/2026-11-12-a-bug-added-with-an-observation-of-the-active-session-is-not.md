---
kind: bug
id: 2026-11-12-a-bug-added-with-an-observation-of-the-active-session-is-not
aliases: []
title: A bug added with an observation of the active session is not in that session's checkpoint or story
status: verified
classification: product-defect
scope: in-scope
impact: A session's story loses the bugs it found (t38k F8)
criteria: []
requirements: [REQ-cli-012]
cases: [TC-cli-bug-observation-joins-the-session]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-observation-joins-the-session
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-observation-joins-the-session-736ba4
receipts: []
environment: local-macos-arm64
revision: 9b9bc010bd5bdf3142ecaa618a7c1ca2e247ecfb
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report, F8 (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:51:00Z
updated: 2026-11-12T20:56:45Z
---
# A bug added with an observation of the active session is not in that session's checkpoint or story

**What happened:** chough bug add and bug update --observation link a bug to an evidence entry of the active session but leave the session's checkpoint bugs empty, so the archived story does not name the bugs found in it
Actual: Checkpoint bugs []

**Steps:** Trigger: Record a failed observation in the active session, then chough bug add --observation <it>
1. Run a case that fails in the active session
2. chough bug add ... --observation <that entry>
3. Read the checkpoint's bugs

**Expected:** The bug is in the session's checkpoint and its archived story

**Why it matters:** A session's story loses the bugs it found (t38k F8)

**Fix:** 7d33d12d (t38c's 9069d1fc): bug add and bug update given --observation with an entry of the active session add the bug to its checkpoint, so its story names it (REQ-cli-012)

## Log

- 2026-11-12T20:51:00Z · Reported (t38k's second fresh-eyes round report, F8 (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-observation-joins-the-session · by claude-code (hc t382)
- 2026-11-12T20:56:45Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-observation-joins-the-session-736ba4; fix: 7d33d12d (t38c's 9069d1fc): bug add and bug update given --observation with an entry of the active session add the bug to its checkpoint, so its story names it (REQ-cli-012). Red at 9b9bc010, green at 68c7a32a · by claude-code (hc t382)
