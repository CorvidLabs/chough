---
kind: bug
id: 2026-11-12-the-accept-steps-plain-specsync-change-check-is-recorded-as-a
aliases: []
title: The accept steps' plain specsync change check is recorded as a direct write, after the review gate
status: verified
classification: product-defect
scope: in-scope
impact: The story blames the workflow's own step and reads out of order (t38k F9)
criteria: []
requirements: [REQ-cli-060]
cases: [TC-cli-gate-close-records-direct-writes-first, TC-cli-accept-step-checks-through-chough]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-records-direct-writes-first
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-step-checks-through-chough
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-records-direct-writes-first-3a2c8c
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-step-checks-through-chough-6cbf4b
receipts: []
environment: local-macos-arm64
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
found_by: claude-code (hc t382) (Fresh-eyes round 2 (t38k, and t38h for Kimi 6; relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:06:25Z
updated: 2026-11-12T21:07:59Z
---
# The accept steps' plain specsync change check is recorded as a direct write, after the review gate

**What happened:** accept.md step 1 runs plain specsync change check, whose lifecycle writes Chough records as changed outside Chough; and the direct-native-write entry is written at the next stage move, after the review gate's gate-closed entry, so the story reads out of order
Actual: direct-native-write after gate-closed, for a step the workflow itself named

**Steps:** Trigger: Follow accept.md as written, close the review gate, then ship
1. specsync change check <id> per accept.md step 1
2. Close the review gate
3. Move to done and read the Timeline

**Expected:** The check runs through Chough and is not flagged; a real direct write is recorded before the gate-closed entry

**Why it matters:** The story blames the workflow's own step and reads out of order (t38k F9)

**Fix:** a28a9455 (t38b's 39146fdc): accept.md's first check runs as chough change check <id> --no-commit, and a gate close records pending direct writes before its gate-closed entry (REQ-cli-057, REQ-cli-060)

## Log

- 2026-11-12T21:06:25Z · Reported (Fresh-eyes round 2 (t38k, and t38h for Kimi 6; relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-records-direct-writes-first, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-step-checks-through-chough · by claude-code (hc t382)
- 2026-11-12T21:07:59Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-gate-close-records-direct-writes-first-3a2c8c, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-step-checks-through-chough-6cbf4b; fix: a28a9455 (t38b's 39146fdc): accept.md's first check runs as chough change check <id> --no-commit, and a gate close records pending direct writes before its gate-closed entry (REQ-cli-057, REQ-cli-060). Red at 804bdb50, green at 91c3824f · by claude-code (hc t382)
