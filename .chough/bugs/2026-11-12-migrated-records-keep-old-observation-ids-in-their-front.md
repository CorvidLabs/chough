---
kind: bug
id: 2026-11-12-migrated-records-keep-old-observation-ids-in-their-front
aliases: []
title: Migrated records keep old observation IDs in their front matter
status: open
classification: product-defect
scope: out-of-scope
impact: Migration polish (t38n F9)
criteria: []
requirements: [REQ-records-009]
cases: []
evidence:
  - 2026-11-13-record-polish/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries
receipts: []
environment: local-macos-arm64
revision: fe6d0f857ebee5f609fc990c8eb89fc87cf1e217
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:36:07Z
updated: 2026-11-13T03:12:11Z
---
# Migrated records keep old observation IDs in their front matter

**What happened:** After migration, a session's observations and a bug's evidence keep OBS- IDs in front matter while the body links the new file, and next lists the old ID; it resolves through the alias, but a reader cannot find the file by that name
Actual: Old IDs in front matter

**Steps:** Trigger: Migrate a 0.5.x tree and read a migrated session and bug
1. chough migrate --apply <digest> with consent
2. Read the front matter

**Expected:** The new entry names, the old IDs kept as aliases

**Why it matters:** Migration polish (t38n F9)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:36:07Z · Reported (t38n's fourth fresh-eyes round report (relayed by t370)) · by claude-code (hc t382)
- 2026-11-12T23:36:08Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Fix 4 real ones, then release (Recommended)', whose option text says 'Polish and the <redacted:host-name> leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester.' (coordinator chat, Claude Code hc t370, 2026-11-12T23:32Z, relayed by claude-code-t370) · by claude-code (hc t382)
- 2026-11-13T03:12:11Z · status: deferred -> open; evidence: 2026-11-13-record-polish/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries. Taken up in 0.6.1 PR 5 on Leif's 'Yes, start (Recommended)' (02:03Z, relayed by t370); red at f4cfb57b · by claude-code (hc t382)
