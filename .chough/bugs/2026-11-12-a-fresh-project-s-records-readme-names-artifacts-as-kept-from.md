---
kind: bug
id: 2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from
aliases: []
title: A fresh project's records README names artifacts as kept from 0.5.x, and the README init paragraph is long
status: open
classification: product-defect
scope: out-of-scope
impact: Docs polish (t38n F11)
criteria: []
requirements: [REQ-records-002]
cases: []
evidence:
  - 2026-11-13-record-polish/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories
  - 2026-11-13-record-polish/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:08Z
updated: 2026-11-13T03:12:11Z
---
# A fresh project's records README names artifacts as kept from 0.5.x, and the README init paragraph is long

**What happened:** chough init writes .chough/README.md listing artifacts/ as 'kept from Chough 0.5.x' in a project that never had 0.5.x, and the README's init paragraph carries a 10-line parenthetical about hosts
Actual: A 0.5.x note; a long parenthetical

**Steps:** Trigger: chough init a new project and read .chough/README.md
1. chough init --objective <o>
2. Read .chough/README.md and the README init paragraph

**Expected:** No 0.5.x note in a fresh project; a short init paragraph

**Why it matters:** Docs polish (t38n F11)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:36:08Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:10Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T03:12:11Z · status: deferred -> open; evidence: 2026-11-13-record-polish/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories, 2026-11-13-record-polish/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept. Taken up in 0.6.1 PR 5 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at f4cfb57b · by claude-code (hc t382)
