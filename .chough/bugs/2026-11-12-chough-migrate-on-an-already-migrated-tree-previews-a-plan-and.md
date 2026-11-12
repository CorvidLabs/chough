---
kind: bug
id: 2026-11-12-chough-migrate-on-an-already-migrated-tree-previews-a-plan-and
aliases: []
title: chough migrate on an already-migrated tree previews a plan and a digest that apply then calls nothing to do
status: open
classification: product-defect
scope: in-scope
impact: A user is sent to apply a plan that does nothing (t38k F10)
criteria: []
requirements: [REQ-records-009]
cases: [TC-cli-migrate-preview-with-nothing-to-migrate]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-cli-migrate-preview-with-nothing-to-migrate]
receipts: []
environment: local-macos-arm64
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:25:59Z
updated: 2026-11-12T20:25:59Z
---
# chough migrate on an already-migrated tree previews a plan and a digest that apply then calls nothing to do

**What happened:** On a tree with no 0.5.x record, chough migrate prints a plan with a plan digest, and --apply with that digest answers nothing_to_do
Actual: A plan and a new digest, then nothing_to_do on apply

**Steps:** Trigger: Run chough migrate on a project already migrated to 0.6.0
1. Migrate a 0.5.x project with chough migrate --apply
2. Run chough migrate again

**Expected:** The preview says there is nothing to migrate and gives no digest to apply

**Why it matters:** A user is sent to apply a plan that does nothing (t38k F10)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T20:25:59Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-migrate-preview-with-nothing-to-migrate · by claude-code (hc t382)
