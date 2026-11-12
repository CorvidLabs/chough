---
kind: bug
id: 2026-11-12-the-scope-gate-summary-ignores-uncommitted-change-files
aliases: []
title: The scope gate summary ignores uncommitted change files
status: open
classification: product-defect
scope: in-scope
impact: The human sees an empty scope at the scope gate (t38n F2)
criteria: []
requirements: [REQ-cli-061]
cases: [TC-cli-scope-gate-summary-counts-uncommitted-change-files]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-scope-gate-summary-counts-uncommitted-change-files
receipts: []
environment: local-macos-arm64
revision: 730aba9f2157cb64d9df9c89788b5df975ad6e9d
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report, F2 (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:48:41Z
updated: 2026-11-12T23:48:41Z
---
# The scope gate summary ignores uncommitted change files

**What happened:** With the delta, plan and other change files written but not committed, chough gate list printed 'What changed: 0 files, +0 −0 since <base>' and named no delta; after git commit it named 8 files and the delta
Actual: 0 files and no delta

**Steps:** Trigger: Write a change's delta and plan, then list gates before committing
1. Write .specsync/changes/<id>/deltas/<module>.md and plan.md
2. chough gate list

**Expected:** The working tree counted, uncommitted and untracked files included, the delta first

**Why it matters:** The human sees an empty scope at the scope gate (t38n F2)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:48:41Z · Reported (t38n's fourth fresh-eyes round report, F2 (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-scope-gate-summary-counts-uncommitted-change-files · by claude-code (hc t382)
- 2026-11-12T23:48:41Z · Added despite similar reports: The verified gate-summary bugs fixed the base commit, the delta's place and the intent; this one is a summary that counts only committed files, so an uncommitted change shows an empty scope · by claude-code (hc t382)
