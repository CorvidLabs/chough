---
kind: bug
id: 2026-11-13-at-the-400-observation-cap-cases-run-writes-its-evidence-entry
aliases: []
title: At the 400-observation cap, cases run writes its evidence entry and then refuses, leaving it unreferenced
status: deferred
classification: product-defect
scope: in-scope
impact: Records change on a refused write, and next can count an unlinked run as current; reached only by a very long session
criteria: []
requirements: [REQ-cli-032]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: fd5397bb391b4a98db91f150e3979ebec4844955
found_by: claude-code (hc t382) (Found by t382 rerunning the referenced cases for Leif's review, 2026-11-13)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T01:24:58Z
updated: 2026-11-13T01:26:12Z
---
# At the 400-observation cap, cases run writes its evidence entry and then refuses, leaving it unreferenced

**What happened:** With the session's observations list at 400, chough cases run executes the case, writes its evidence entry under evidence/, then refuses with 'too many observation (limit 400)'; the entry stays in the session folder unreferenced, and the refusal names no recovery, though REQ-cli-032 says such a refusal leaves every record byte-identical and names chough session shelve <id>; next then reads the unreferenced entry as the case's latest run
Actual: An unreferenced evidence entry written; 'too many observation (limit 400)'

**Steps:** Trigger: Run a case in a session whose observations list holds 400 entries
1. Fill a session's observations to 400 (here, by four rounds of reruns)
2. chough cases run <case> --environment <env>
3. Look at evidence/ and the checkpoint

**Expected:** Refused before anything is written, naming the bound and chough session shelve <id>

**Why it matters:** Records change on a refused write, and next can count an unlinked run as current; reached only by a very long session

**Fix:** Not fixed yet.

## Log

- 2026-11-13T01:24:58Z · Reported (Found by t382 rerunning the referenced cases for Leif's review, 2026-11-13) · by claude-code (hc t382)
- 2026-11-13T01:26:12Z · status: open -> deferred. Deferred to 0.6.1 by Leif's decision: 'Defer to 0.6.1, ship (Recommended)', whose option text says 'Recorded as deferred with your words. t382 ships now, then CI, merge it and the release go. It only affects very long sessions, and nothing is lost.' (coordinator chat, Claude Code hc t370, 2026-11-13T01:25Z, relayed by claude-code-t370) · by claude-code (hc t382)
