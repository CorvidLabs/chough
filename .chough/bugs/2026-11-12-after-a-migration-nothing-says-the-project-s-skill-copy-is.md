---
kind: bug
id: 2026-11-12-after-a-migration-nothing-says-the-project-s-skill-copy-is
aliases: []
title: After a migration nothing says the project's skill copy is still 0.5.x
status: open
classification: product-defect
scope: in-scope
impact: An agent follows removed commands after the upgrade (t38n F3)
criteria: []
requirements: [REQ-records-009]
cases: [TC-records-migration-names-skill-update-for-an-outdated-copy]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migration-names-skill-update-for-an-outdated-copy
receipts: []
environment: local-macos-arm64
revision: 730aba9f2157cb64d9df9c89788b5df975ad6e9d
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report, F3 (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:48:48Z
updated: 2026-11-12T23:48:48Z
---
# After a migration nothing says the project's skill copy is still 0.5.x

**What happened:** After chough migrate --apply, chough skill status reports most of the installed skill files known-outdated, and the copy still names removed commands and paths, yet neither the plan's nor the receipt's follow-ups, next, doctor nor the upgrade steps mention chough skill update
Actual: No pointer; the agent keeps reading 0.5.x instructions

**Steps:** Trigger: Migrate a 0.5.x project whose installed skill copy is a 0.5.x release
1. chough migrate, then chough migrate --apply <digest> with consent
2. chough skill status

**Expected:** The follow-ups name chough skill update while a copy is not current

**Why it matters:** An agent follows removed commands after the upgrade (t38n F3)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:48:48Z · Reported (t38n's fourth fresh-eyes round report, F3 (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-migration-names-skill-update-for-an-outdated-copy · by claude-code (hc t382)
