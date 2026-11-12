---
kind: bug
id: 2026-11-12-at-plan-after-approval-next-shows-specsync-s-next-step-as-if
aliases: []
title: At plan after approval, next shows SpecSync's next step as if it were the step to take
status: verified
classification: product-defect
scope: in-scope
impact: An agent may run the check before building (t38m F8)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-next-shows-specsyncs-step-for-reference-only]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only-0e76c0
receipts: []
environment: local-macos-arm64
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:57:05Z
updated: 2026-11-12T23:02:45Z
---
# At plan after approval, next shows SpecSync's next step as if it were the step to take

**What happened:** chough next at plan after the scope approval shows 'Native next action: run specsync change check <id> --commit' beside Chough's 'Next: … chough stage advance', two actions that disagree
Actual: Two next actions that disagree

**Steps:** Trigger: Approve the scope, then run chough next at plan
1. Close the scope gate with the human's approval
2. chough next

**Expected:** SpecSync's step labelled for reference, the action to take being Chough's Next

**Why it matters:** An agent may run the check before building (t38m F8)

**Fix:** 81079b63 (t37f's 94e432cc): next labels SpecSync's own next step for reference, the step to take now being Next

## Log

- 2026-11-12T22:57:05Z · Reported (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only · by claude-code (hc t382)
- 2026-11-12T22:57:05Z · Added despite similar reports: The verified bug is SpecSync's review command at accept; this one is SpecSync's change check step at plan after the approval, shown as a second next action · by claude-code (hc t382)
- 2026-11-12T23:02:45Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only-0e76c0; fix: 81079b63 (t37f's 94e432cc): next labels SpecSync's own next step for reference, the step to take now being Next. Red at 7fb91195, green at e0131c09 · by claude-code (hc t382)
