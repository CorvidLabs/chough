---
kind: bug
id: 2026-11-12-the-intent-gate-summary-names-criterion-ids-without-their
aliases: []
title: The intent gate summary names criterion IDs without their sentences and says the diff is unknown
status: verified
classification: product-defect
scope: out-of-scope
impact: Gate summary polish (t38n F7)
criteria: []
requirements: [REQ-cli-061]
cases: []
evidence:
  - 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence
  - 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies
  - 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence-7a2b2c
  - 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies-5bbd77
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:24Z
updated: 2026-11-13T02:26:00Z
---
# The intent gate summary names criterion IDs without their sentences and says the diff is unknown

**What happened:** chough gate list at an intent gate shows 'Intent: TALLY-2' and the HI file with no sentence being approved, prints 'Diff: unknown, with no merge base and no recorded base commit', and an ADDED-only delta reads 'adds or modifies'
Actual: IDs only, an unknown diff, 'adds or modifies'

**Steps:** Trigger: List gates at an intent gate, and at a scope gate whose delta only adds
1. chough gate list

**Expected:** The criterion sentences, no diff line where none applies, 'adds' for an ADDED-only delta

**Why it matters:** Gate summary polish (t38n F7)

**Fix:** 8e0feede (t37f's 785ead6f): at an intent gate with no native change the summary gives each criterion with its sentence and no diff line, and a delta says what it adds and what it modifies (REQ-cli-061)

## Log

- 2026-11-12T23:36:24Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:24Z · Added despite similar reports: The verified bugs fixed an unusable git diff line and added the intent's criterion IDs; this one is the sentences missing beside those IDs, a diff line where none applies, and 'adds or modifies' for an ADDED-only delta · by claude-code (hc t382)
- 2026-11-12T23:36:26Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T02:25:28Z · status: deferred -> open; evidence: 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence, 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies. Taken up in 0.6.1 PR 4 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at 0f34dbe2 · by claude-code (hc t382)
- 2026-11-13T02:26:00Z · status: open -> verified; evidence: 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence-7a2b2c, 2026-11-13-intent-gate-summary/2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies-5bbd77; fix: 8e0feede (t37f's 785ead6f): at an intent gate with no native change the summary gives each criterion with its sentence and no diff line, and a delta says what it adds and what it modifies (REQ-cli-061). Red at 0f34dbe2, green at 8e0feede · by claude-code (hc t382)
