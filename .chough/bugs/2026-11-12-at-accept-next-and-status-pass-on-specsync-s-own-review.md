---
kind: bug
id: 2026-11-12-at-accept-next-and-status-pass-on-specsync-s-own-review
aliases: []
title: At accept, next and status pass on SpecSync's own review command
status: verified
classification: product-defect
scope: in-scope
impact: An agent is routed to run the review itself, the step only the human's decision drives (t38m F1)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-at-accept-never-passes-on-the-review-command]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-never-passes-on-the-review-command
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-never-passes-on-the-review-command-11d611
receipts: []
environment: local-macos-arm64
revision: 24f9ca8d6b42d6075bd2e529272aa9904f44cb31
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F1 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:53:10Z
updated: 2026-11-12T22:02:31Z
---
# At accept, next and status pass on SpecSync's own review command

**What happened:** With the change verifying and the review gate open, chough next and status print SpecSync's native next action, 'run specsync change review <id> --reviewer <human> after the PR's scoped review passes', under Native change and Gate to done, though Chough masks the approve command at plan
Actual: The review command is printed twice, once in the native summary and once in the gate to done

**Steps:** Trigger: Reach accept with the change verifying and the review gate open, then run chough next
1. Advance to accept and run chough change check
2. Open the review gate
3. chough next --environment <env>

**Expected:** No field names specsync change review; the native action reads as the human's review at the review gate

**Why it matters:** An agent is routed to run the review itself, the step only the human's decision drives (t38m F1)

**Fix:** 5b6a7732: next and status show SpecSync's review command at accept as the human's review at the review gate, never as a command for the agent (REQ-cli-021)

## Log

- 2026-11-12T21:53:10Z · Reported (t38m's third fresh-eyes round report, F1 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-never-passes-on-the-review-command · by claude-code (hc t382)
- 2026-11-12T21:53:10Z · Added despite similar reports: Earlier bugs covered next's own accept guidance (#76) and next at plan passing on the approve command; this is SpecSync's native next action at accept, shown in the native summary and the gate to done, naming its review command · by claude-code (hc t382)
- 2026-11-12T22:02:31Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-accept-never-passes-on-the-review-command-11d611; fix: 5b6a7732: next and status show SpecSync's review command at accept as the human's review at the review gate, never as a command for the agent (REQ-cli-021). Red at 24f9ca8d, green at 5b6a7732 · by claude-code (hc t382)
