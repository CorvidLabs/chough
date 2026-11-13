---
kind: bug
id: 2026-11-12-next-with-no-native-change-says-none-recorded-read-observed
aliases: []
title: next with no native change says none recorded (read observed)
status: verified
classification: product-defect
scope: in-scope
impact: A cryptic read state where there is nothing to read (t38k F12)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-with-no-native-change-says-only-none-recorded]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded-30b630
receipts: []
environment: local-macos-arm64
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:02:31Z
updated: 2026-11-12T21:04:01Z
---
# next with no native change says none recorded (read observed)

**What happened:** chough next prints 'Native change: none recorded (read observed)' when no native change is recorded
Actual: Native change: none recorded (read observed)

**Steps:** Trigger: Run chough next in a session with no native change
1. chough init
2. chough next

**Expected:** Native change: none recorded

**Why it matters:** A cryptic read state where there is nothing to read (t38k F12)

**Fix:** 7cbc20c4 (t37f's 1a1e6c06): next with no native change says only 'Native change: none recorded'

## Log

- 2026-11-12T21:02:31Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded · by claude-code (hc t382)
- 2026-11-12T21:04:01Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded-30b630; fix: 7cbc20c4 (t37f's 1a1e6c06): next with no native change says only 'Native change: none recorded'. Red at 5195bd67, green at cca98758 · by claude-code (hc t382)
