---
kind: bug
id: 2026-11-12-outputs-and-docs-disagree-cases-run-has-no-recorded-key-and
aliases: []
title: "Outputs and docs disagree: cases run has no recorded key, and the migrate becomes and unstaged keys are undocumented"
status: investigating
classification: product-defect
scope: blocking
impact: Automation written from the docs breaks
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-name-what-cases-run-returns
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migration-docs-name-every-key
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:41Z
updated: 2026-11-12T16:34:55Z
---
# Outputs and docs disagree: cases run has no recorded key, and the migrate becomes and unstaged keys are undocumented

**What happened:** docs/cli.md says observation and case sit beside recorded in cases run JSON, which has no recorded key; the migrate preview's becomes and the apply's unstaged keys are not documented
Actual: Three mismatches

**Steps:** chough cases run; chough migrate and --apply

**Expected:** docs/cli.md matches the output

**Why it matters:** Automation written from the docs breaks

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:41Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T16:34:55Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-docs-name-what-cases-run-returns, 2026-11-12-release-0-6-0/2026-11-12-tc-records-migration-docs-name-every-key. Reproduced: red at e005da2e, TC-cli-docs-name-what-cases-run-returns (the cases row says nothing of what run returns) and TC-records-migration-docs-name-every-key (the preview's apply key is not named) failed. In scope (Leif, 16:26Z, relayed by t370) · by claude-code (hc t382)
