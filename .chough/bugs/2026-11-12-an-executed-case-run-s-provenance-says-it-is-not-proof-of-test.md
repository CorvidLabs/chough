---
kind: bug
id: 2026-11-12-an-executed-case-run-s-provenance-says-it-is-not-proof-of-test
aliases: []
title: An executed case run's provenance says it is not proof of test execution
status: verified
classification: product-defect
scope: in-scope
impact: The evidence contradicts itself and undercuts real test runs (t38k F12)
criteria: []
requirements: [REQ-cli-011]
cases: [TC-cli-executed-case-run-provenance]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-executed-case-run-provenance
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-executed-case-run-provenance-0e0ebc
receipts: []
environment: local-macos-arm64
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:25:59Z
updated: 2026-11-12T20:31:18Z
---
# An executed case run's provenance says it is not proof of test execution

**What happened:** chough cases run records a passing automated run with the provenance 'unauthenticated observation; not proof of human approval, test execution, or deployment', though Chough ran the test
Actual: The executed run denies test execution

**Steps:** Trigger: Run an automated case with chough cases run and read its provenance
1. chough cases run <case> --environment <env>
2. Read the run's provenance

**Expected:** An executed run says it is not proof of human approval or deployment; the full sentence stays for manual entries

**Why it matters:** The evidence contradicts itself and undercuts real test runs (t38k F12)

**Fix:** 79661189 (t38c's 8d062454): an executed case run's provenance says it is not proof of human approval or deployment; manual entries keep the full sentence (REQ-cli-011)

## Log

- 2026-11-12T20:25:59Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-executed-case-run-provenance · by claude-code (hc t382)
- 2026-11-12T20:31:09Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-executed-case-run-provenance-0e0ebc; fix: t38c's 8d062454: an executed case run's provenance says it is not proof of human approval or deployment; manual entries keep the full sentence (REQ-cli-011). Red at a0152789, green at 34b7cbb7 · by claude-code (hc t382)
- 2026-11-12T20:31:18Z · fix: 79661189 (t38c's 8d062454): an executed case run's provenance says it is not proof of human approval or deployment; manual entries keep the full sentence (REQ-cli-011). Fix line corrected to name the commit on leif/human-first-records · by claude-code (hc t382)
