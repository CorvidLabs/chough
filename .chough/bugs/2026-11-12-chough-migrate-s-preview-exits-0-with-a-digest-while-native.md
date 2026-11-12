---
kind: bug
id: 2026-11-12-chough-migrate-s-preview-exits-0-with-a-digest-while-native
aliases: []
title: chough migrate's preview exits 0 with a digest while native changes are open, and only --apply says it will refuse
status: open
classification: product-defect
scope: out-of-scope
impact: Not yet assessed.
criteria: []
requirements: [REQ-records-009]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: 6bed53d1b3f73c2a4627f956d74dc0f69d322fd0
found_by: claude-code (hc t382) (t38f, dogfood on jackdaw, relayed by t370)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T15:12:25Z
updated: 2026-11-12T15:12:25Z
---
# chough migrate's preview exits 0 with a digest while native changes are open, and only --apply says it will refuse

**What happened:** On jackdaw, with 12 open native changes, chough migrate printed a plan and digest and exited 0; only migrate --apply refused with refused:change-open
Actual: The open-change check runs only in --apply (src/migrate.rs)

**Steps:** chough migrate in a project with any native change not archived

**Expected:** The preview says up front that apply will refuse, naming the open changes (REQ-records-009 refuses the apply while any change is open)

**Why it matters:** Not yet assessed.

**Fix:** Not fixed yet.

## Log

- 2026-11-12T15:12:25Z · Reported (t38f, dogfood on jackdaw, relayed by t370) · by claude-code (hc t382)
