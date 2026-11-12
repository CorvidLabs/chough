---
kind: bug
id: 2026-10-31-build-gate-admits-a-definition-native-status-reports-as
aliases: [BUG-0006]
title: Build gate admits a definition native status reports as invalid
status: verified
classification: product-defect
scope: in-scope
impact: Implementation can start on a scope nobody approved in its current form
criteria: [FLOW-5]
requirements: [REQ-cli-008, REQ-cli-003]
cases: []
evidence: [bug-0006-repro-1, bug-0006-fixed-1]
receipts:
  - id: 2026-10-31-with-native-status-reporting-state-approved-and-approval-valid
    aliases: bug-0006-repro-1
    claim: With native status reporting state approved and approval_valid false, chough stage advance moved plan -> build with gate native:approved
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:0e454e9e238caf4651a960123a0ca2f6d27c2923ecd9af7e47bd3e0c7906b8ab
    observed: 2026-10-31T20:44:38Z
  - id: 2026-10-31-with-approval-valid-false-plan-build-is-refused-as-refused
    aliases: bug-0006-fixed-1
    claim: With approval_valid false, plan -> build is refused as refused:native-approval-invalid and names the native next action
    result: passed
    revision: 80757487bddf9172bd757f78fcfa798e1d7e40b3
    environment: local-macos-arm64
    fingerprint: sha256:9fbd1aeafcb8ef5045a7a5cce68e3d46ea5eebba3d81d35166303488452b96d5
    observed: 2026-10-31T21:24:00Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding A, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:00Z
updated: 2026-10-31T21:24:01Z
---
# Build gate admits a definition native status reports as invalid

**What happened:** plan -> build succeeded with gate native:approved while native status reported approval_valid false
Actual: Allowed: the gate compares summary.state only

**Steps:** Approved change whose definition no longer matches its approval
1. Approve a change, then change its scope
2. specsync change status <id> --json reports approval_valid false
3. chough stage advance from plan

**Expected:** Entering build is refused and names the native next action (re-approve)

**Why it matters:** Implementation can start on a scope nobody approved in its current form

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:00Z · Reported (Review finding A, reproduced 2026-10-31 against main f19823d); evidence: bug-0006-repro-1
- 2026-10-31T21:24:01Z · status: open -> verified. Fixed in 8075748 (fail-closed stage gates); regression TC-cli-0040 passed (observation tc-cli-0040-1793481644).; evidence: bug-0006-fixed-1
