---
kind: bug
id: 2026-11-12-the-gate-list-footer-offers-gate-record-for-a-verifying-gate
aliases: []
title: The gate list footer offers gate record for a verifying gate, whose refusal gives a wrong reason
status: verified
classification: product-defect
scope: in-scope
impact: The human's reply path is a dead end (t38k F6)
criteria: []
requirements: [REQ-cli-061]
cases: [TC-cli-gate-list-names-the-ship-for-a-verifying-gate]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-21e743
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:37Z
updated: 2026-11-12T21:04:01Z
---
# The gate list footer offers gate record for a verifying gate, whose refusal gives a wrong reason

**What happened:** chough gate list ends with the gate record reply for every gate, though gate record refuses a gate whose change is verifying, saying nothing may write .chough/ before it ships, which verification_ignored_paths contradicts
Actual: gate record offered, then refused with a reason that contradicts the policy

**Steps:** Trigger: List gates while the review gate's change is verifying
1. Open the review gate at accept with the change verifying
2. chough gate list
3. chough gate record <list> --gates <n> ...

**Expected:** The list names change ship --list --gate for that gate; the refusal says the gate closes through the ship

**Why it matters:** The human's reply path is a dead end (t38k F6)

**Fix:** cca98758 (t37f's c5423ca4): the list names change ship --list --gate for a verifying gate, and gate record's refusal says it closes through the ship (REQ-cli-061)

## Log

- 2026-11-12T21:02:37Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate · by claude-code (hc t382)
- 2026-11-12T21:04:01Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-21e743; fix: cca98758 (t37f's c5423ca4): the list names change ship --list --gate for a verifying gate, and gate record's refusal says it closes through the ship (REQ-cli-061). Red at 5195bd67, green at cca98758 · by claude-code (hc t382)
