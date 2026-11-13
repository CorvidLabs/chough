---
kind: bug
id: 2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one
aliases: []
title: A bug body runs its symptom and its Actual line into one paragraph
status: verified
classification: product-defect
scope: out-of-scope
impact: Record polish (t38n F8)
criteria: []
requirements: [REQ-cli-012]
cases: []
evidence:
  - 2026-11-13-record-polish/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
  - 2026-11-13-record-polish/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:05Z
updated: 2026-11-13T03:12:46Z
---
# A bug body runs its symptom and its Actual line into one paragraph

**What happened:** A bug's body renders '**What happened:** <symptom>' followed by 'Actual: 1' on the next line, which Markdown joins into one paragraph
Actual: One run-on paragraph

**Steps:** Trigger: Add a bug with --symptom and --actual and render its Markdown
1. chough bug add ... --symptom <s> --actual <a>

**Expected:** The actual result on its own line or paragraph

**Why it matters:** Record polish (t38n F8)

**Fix:** t38c's 6d6a53a4: a bug body gives 'Actual: <a>' its own paragraph after What happened; bugs written before 0.6.1 read the same (REQ-cli-012)

## Log

- 2026-11-12T23:36:05Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:07Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T03:12:11Z · status: deferred -> open; evidence: 2026-11-13-record-polish/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph. Taken up in 0.6.1 PR 5 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at f4cfb57b · by claude-code (hc t382)
- 2026-11-13T03:12:46Z · status: open -> verified; evidence: 2026-11-13-record-polish/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb; fix: t38c's 6d6a53a4: a bug body gives 'Actual: <a>' its own paragraph after What happened; bugs written before 0.6.1 read the same (REQ-cli-012). Green at 6b7cd476 · by claude-code (hc t382)
