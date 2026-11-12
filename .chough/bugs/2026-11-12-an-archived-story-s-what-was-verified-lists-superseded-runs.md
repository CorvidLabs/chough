---
kind: bug
id: 2026-11-12-an-archived-story-s-what-was-verified-lists-superseded-runs
aliases: []
title: An archived story's What was verified lists superseded runs and repeats each command
status: open
classification: product-defect
scope: in-scope
impact: The story buries what was verified last (t38k F8)
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-story-lists-each-cases-latest-run]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-history-story-lists-each-cases-latest-run]
receipts: []
environment: local-macos-arm64
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
found_by: claude-code (hc t382) (Fresh-eyes round 2 (t38k F8; relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:06:33Z
updated: 2026-11-12T21:06:33Z
---
# An archived story's What was verified lists superseded runs and repeats each command

**What happened:** At the move into done the story's What was verified gets one line per receipt, every earlier run of a case included, and each line repeats the command the claim already holds
Actual: Every run listed, each command twice

**Steps:** Trigger: Run one case three times in a session, then archive it
1. chough cases run <case> three times
2. Move the session into done
3. Read What was verified

**Expected:** The latest run of each case and each manual entry, each command once, with a count of the earlier runs

**Why it matters:** The story buries what was verified last (t38k F8)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T21:06:33Z · Reported (Fresh-eyes round 2 (t38k F8; relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-lists-each-cases-latest-run · by claude-code (hc t382)
- 2026-11-12T21:06:33Z · Added despite similar reports: The verified bug fixed a bare-ID duplicate of every entry; this one is superseded runs of one case listed beside its latest, and the command repeated inside one line · by claude-code (hc t382)
