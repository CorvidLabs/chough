---
kind: bug
id: 2026-11-12-the-gate-summary-s-what-changed-counts-the-whole-branch-since
aliases: []
title: The gate summary's What changed counts the whole branch since the default branch, overstating a second change on one branch
status: open
classification: requirement-gap
scope: out-of-scope
impact: Not yet assessed.
criteria: []
requirements: [REQ-cli-061]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: f937e7916219f9a9749b2867b9026ae2e5f40721
found_by: "claude-code (hc t382) (t382, dogfood of the 0.6.0 release-prep change; deferred by Leif: 'Fix after 0.6.0' (2026-11-12T14:46Z, relayed by t370))"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T14:47:11Z
updated: 2026-11-12T14:47:11Z
---
# The gate summary's What changed counts the whole branch since the default branch, overstating a second change on one branch

**What happened:** For a second native change on a branch whose first change is archived, chough gate list's What changed lists every file since the merge base with main, including the first change's work
Actual: REQ-cli-061 specifies the merge base with the default branch first, so the summary follows the contract and overstates

**Steps:** Open a second native change on a branch with an archived first change, then chough gate list

**Expected:** The summary counts only the change's own work, from its base_commit (needs a REQ-cli-061 delta)

**Why it matters:** Not yet assessed.

**Fix:** Not fixed yet.

## Log

- 2026-11-12T14:47:11Z · Reported (t382, dogfood of the 0.6.0 release-prep change; deferred by Leif: 'Fix after 0.6.0' (2026-11-12T14:46Z, relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T14:47:11Z · Added despite similar reports: #190 fixed which files a summary counts (Chough's records and files merged from main); this is which base it counts from for a second change on one branch, which REQ-cli-061 itself specifies · by claude-code (hc t382)
