---
kind: bug
id: 2026-11-12-chough-doctor-reports-a-case-definition-s-deliberate-temporary
aliases: []
title: chough doctor reports a case definition's deliberate temporary-root test setting as a leak and suggests --redact, which would break the case
status: investigating
classification: requirement-gap
scope: blocking
impact: The verify and CI lanes fail on the finding, and following the suggested fix breaks the project's cases
criteria: [LIFE-26]
requirements: [REQ-records-006]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-reproduced-the-jackdaw-finding-t38f-on-a-rehearsal-migrated
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks
receipts: []
environment: local-macos-arm64
revision: 8d310373fdb232f3055c710add18018a97dcd3a7
found_by: claude-code (hc t382) (t38f on jackdaw, relayed by t370 (2026-11-12); reproduced by t382)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T15:40:55Z
updated: 2026-11-12T16:04:18Z
---
# chough doctor reports a case definition's deliberate temporary-root test setting as a leak and suggests --redact, which would break the case

**What happened:** On a rehearsal-migrated jackdaw tree, chough doctor reported 4 leak findings (match system-path) for case files that set XDG_DATA_HOME under the temporary root, and suggested chough doctor --fix --redact
Actual: chough doctor reports a leak (system-path) naming the case file and line, and its fix, --fix --redact, would replace the setting with a redaction marker and break the case

**Steps:** A committed .chough/cases/*.cases.toml whose binding sets an environment value to a path under the temporary root
1. Commit a case file whose command binding runs env XDG_DATA_HOME=<tmp>/probe-data true
2. Run chough doctor

**Expected:** A case definition's deliberate temporary-root test setting is not reported as a leak; home paths, user names and credentials in it still are

**Why it matters:** The verify and CI lanes fail on the finding, and following the suggested fix breaks the project's cases

**Fix:** Not fixed yet.

## Log

- 2026-11-12T15:40:55Z · Reported (t38f on jackdaw, relayed by t370 (2026-11-12); reproduced by t382); evidence: 2026-11-12-release-0-6-0/2026-11-12-reproduced-the-jackdaw-finding-t38f-on-a-rehearsal-migrated · by claude-code (hc t382)
- 2026-11-12T15:41:07Z · t38f's report (claude-code, claude-opus-5-5, hc t38f, 2026-11-12): CorvidLabs/jackdaw, a throwaway worktree migrated with chough 0.6.0 at 6bed53d; chough doctor exits 1 with 4 leak findings (system-path, fix --fix --redact) at .chough/cases/match-rules.cases.toml:65, state-machine.cases.toml:65 and :108, and table-template.cases.toml:23, each a case binding's env = { XDG_DATA_HOME = <a path under the temporary root> } that isolates the Godot case runs' data folder. migrate never reads these files (the cases are already 0.6.0 format), so its kept_with_leaks of 0 says nothing about them. Suggested: no leak for a temporary-root path in a case's env, or at least no --redact offer. The fix changes REQ-records-006's text, so t370 is asking Leif (2026-11-12) · by claude-code (hc t382)
- 2026-11-12T16:04:18Z · status: open -> investigating; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks. Leif chose 'Yes (Recommended)' (15:41Z, relayed by t370): temporary-root paths in case definitions are not leaks (REQ-records-006 delta). Red at 423169de: TC-records-doctor-case-temp-paths-are-not-leaks failed at its first assertion, doctor reported leaks for the case file's temporary-root paths · by claude-code (hc t382)
