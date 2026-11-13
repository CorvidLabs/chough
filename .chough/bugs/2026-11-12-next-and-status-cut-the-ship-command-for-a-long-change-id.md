---
kind: bug
id: 2026-11-12-next-and-status-cut-the-ship-command-for-a-long-change-id
aliases: []
title: next and status cut the ship command for a long change ID
status: verified
classification: product-defect
scope: in-scope
impact: An agent copies a broken command (t38k F5)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-and-status-show-every-command-whole]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-show-every-command-whole
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-show-every-command-whole-1869e9
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:35Z
updated: 2026-11-12T21:04:01Z
---
# next and status cut the ship command for a long change ID

**What happened:** With a long change ID, chough next and status cut chough change ship <id> --close-gate mid-command to stay within their text bound
Actual: The ship command is cut

**Steps:** Trigger: Record a native change with a 120-character ID and reach accept while verifying
1. chough next --json

**Expected:** Every command whole; other text is cut instead

**Why it matters:** An agent copies a broken command (t38k F5)

**Fix:** 47fa2b9c (t37f's 7950a0e9): next and status keep every command whole; the bound cuts only the text around commands (REQ-cli-021)

## Log

- 2026-11-12T21:02:35Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-show-every-command-whole · by claude-code (hc t382)
- 2026-11-12T21:04:01Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-and-status-show-every-command-whole-1869e9; fix: 47fa2b9c (t37f's 7950a0e9): next and status keep every command whole; the bound cuts only the text around commands (REQ-cli-021). Red at 5195bd67, green at cca98758 · by claude-code (hc t382)
