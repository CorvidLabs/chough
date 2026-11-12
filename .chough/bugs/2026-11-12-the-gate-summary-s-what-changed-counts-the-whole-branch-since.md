---
kind: bug
id: 2026-11-12-the-gate-summary-s-what-changed-counts-the-whole-branch-since
aliases: []
title: The gate summary's What changed counts the whole branch since the default branch, overstating a second change on one branch
status: verified
classification: requirement-gap
scope: blocking
impact: Not yet assessed.
criteria: []
requirements: [REQ-cli-061]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base-afcf46
receipts: []
environment: local-macos-arm64
revision: f937e7916219f9a9749b2867b9026ae2e5f40721
found_by: "claude-code (hc t382) (t382, dogfood of the 0.6.0 release-prep change; deferred by Leif: 'Fix after 0.6.0' (2026-11-12T14:46Z, relayed by t370))"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T14:47:11Z
updated: 2026-11-12T15:29:28Z
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
- 2026-11-12T15:27:54Z · status: open -> investigating; scope: out-of-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base. In scope for 0.6.0 by Leif's 'Block the tag' (2026-11-12T15:14Z, relayed by t370) and the re-scoped definition he approved at 15:24Z (REQ-cli-061 delta: count from the change's own base_commit). Red at acd2b0b0: TC-cli-gate-summary-counts-from-the-changes-own-base failed, the summary said 'What changed: 4 files, +4 −0' from the merge base where 1 file was expected · by claude-code (hc t382)
- 2026-11-12T15:29:28Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base-afcf46. Green at d5381eef: TC-cli-gate-summary-counts-from-the-changes-own-base passed; #190's TC-cli-190-gate-summary-counts-the-change and G2's TC-cli-gate-summary-says-when-there-is-no-base also passed at the same commit · by claude-code (hc t382)
