---
kind: bug
id: 2026-11-02-accepted-first-fixture-dependency-prevents-audit-qualification
aliases:
  - BUG-accepted-first-fixture-dependency-prevents-audit-qualification-ba464069b0ad48c7070a11ca
title: Accepted-first fixture dependency prevents audit qualification
status: verified
classification: requirement-gap
scope: blocking
impact: Blocks full verify, native closeout and merge qualification despite passing product tests.
criteria: [FLOW-5, FLOW-10]
requirements: [REQ-cli-003]
cases: []
evidence: [OBS-structural-d607388cf7cd0eb18ee64eef, OBS-structural-c07c15026bd2fd81c2773bf3]
receipts:
  - id: 2026-11-02-native-qualification-is-blocked-by-the-accepted-first-fixture
    aliases: OBS-structural-d607388cf7cd0eb18ee64eef
    claim: "Native qualification is blocked by the accepted-first fixture dependency: change check refused exit 1 while the parent is verifying; full verify audit rejected tests/next.rs coverage at step 6."
    result: failed
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:93220b6c130f6ed6e46db4a011031d544d6400f0ceb75d1950dd727279fe599d
    observed: 2026-11-02T01:43:23Z
  - id: 2026-11-02-after-actual-human-approval-the-identity-parent-was-natively
    aliases: OBS-structural-c07c15026bd2fd81c2773bf3
    claim: After actual human approval, the identity parent was natively archived, the fixture check became available, and full Fledge verify passed all 9 steps in 156.372 seconds, including audit and 182 unfiltered tests.
    result: passed
    revision: 7dca7a700f18ba2091bd0a199610bf53fd19a1a2
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:f7bc3f2e9916eef4eee382557e880f9556a9b01480053e91b3c03cd8f4cf79ac
    observed: 2026-11-02T02:04:41Z
environment: local-macos-arm64
revision: b118422c826b29a41df2eea38c58120f6dbc77ae
found_by: Observed takeover native-ordering error
related: []
created: 2026-11-02T01:43:24Z
updated: 2026-11-02T02:04:42Z
---
# Accepted-first fixture dependency prevents audit qualification

**What happened:** An approved same-PR fixture follow-up does not count as native coverage because its declared dependency requires the parent to be accepted first.
Actual: The fixture check refuses until parent acceptance and native audit reports tests/next.rs uncovered; all 182 product tests pass on both toolchains.

**Steps:** Run native fixture check or the verify lane while the approved identity parent is verifying.
1. specsync change check update-compact-next-fixture-for-slug-observations --json
2. fledge lanes run verify

**Expected:** Qualify the parent and its approved fixture together before human acceptance, using an ordering plan supported by native tooling.

**Why it matters:** Blocks full verify, native closeout and merge qualification despite passing product tests.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T01:43:24Z · Reported (Observed takeover native-ordering error); evidence: OBS-structural-d607388cf7cd0eb18ee64eef
- 2026-11-02T01:43:24Z · Added despite similar reports: The obsolete observation-ID fixture defect is fixed and has a later passing case. This is a separate planning/recovery gap in the accepted-first native dependency.
- 2026-11-02T02:04:42Z · status: open -> verified. Resolved through the supported sequential native workflow after 0xLeif explicitly approved the implementation: parent review/ship archived it, enabling the dependent fixture check. A strictly later full 9-step verify passes audit. Original refusals remain retained.; evidence: OBS-structural-c07c15026bd2fd81c2773bf3
