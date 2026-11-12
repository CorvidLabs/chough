---
kind: bug
id: 2026-11-12-chough-migrate-apply-can-write-a-session-md-that-ends-with-a
aliases: []
title: chough migrate --apply can write a session.md that ends with a blank line, which git diff --check reports
status: open
classification: product-defect
scope: in-scope
impact: A repository whose format check runs git diff --check on committed ranges fails on the migration commit; the file cannot be fixed by hand without breaking the receipt's digest
criteria: []
requirements: [REQ-records-009]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: ad3214229cc6513156f661fbe960eaed929765cb
found_by: claude-code (hc t382) (t38f (claude-code, claude-opus-5-5, hc t38f), the real jackdaw migration, 2026-11-12; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:37:34Z
updated: 2026-11-12T16:37:34Z
---
# chough migrate --apply can write a session.md that ends with a blank line, which git diff --check reports

**What happened:** On jackdaw (build 72511830, plan digest 24a12f8c), migrate --apply wrote .chough/sessions/2026-11-11-issue-89-item-effects-design/session.md ending in two newlines; git diff --cached --check reports a new blank line at EOF on its line 79
Actual: A migrated session.md ends with a blank line

**Steps:** chough migrate --apply on a 0.5.x project whose open session's carried story ends in a way that leaves a trailing blank line

**Expected:** Every record Chough writes ends with exactly one newline

**Why it matters:** A repository whose format check runs git diff --check on committed ranges fails on the migration commit; the file cannot be fixed by hand without breaking the receipt's digest

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:37:34Z · Reported (t38f (claude-code, claude-opus-5-5, hc t38f), the real jackdaw migration, 2026-11-12; recorded by t382, not yet reproduced) · by claude-code (hc t382)
