---
kind: bug
id: 2026-11-02-leaving-test-ignores-a-referenced-case-s-failed-unlinked
aliases:
  - BUG-leaving-test-ignores-a-referenced-case-s-failed-unlinked-observation-ea76be10351c67bc019ac939
title: Leaving test ignores a referenced case's failed, unlinked observation
status: verified
classification: product-defect
scope: in-scope
impact: A known failure is left behind at test; FLOW-7 and LIFE-5 are not enforced
criteria: [FLOW-7, LIFE-5]
requirements: [REQ-cli-008]
cases: []
evidence:
  - OBS-product-test-9e2d612534e636aa16dbc628
  - OBS-product-test-cb90047d10b04254e607a3a5
  - OBS-product-test-11f42ac98b4076b73c7dbb9d
  - OBS-product-test-16dc837a3d844c63a41b6932
  - OBS-tc-cli-a-later-not-run-keeps-an-unlinked-failure-refused-9fe14fa45436050117427a42
receipts:
  - id: 2026-11-02-issue-39-reproduced-at-825a380-in-a-shipping-sample-stage
    aliases: OBS-product-test-9e2d612534e636aa16dbc628
    claim: "Issue #39 reproduced at 825a380: in a shipping sample, stage advance left test for accept (exit 0) while the referenced case TC-shipping-0002's latest observation had failed and no bug linked it; chough next showed no warning"
    result: failed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-linux-x86_64
    content: 156e42275d28bb7f94c835fef2af188ad6e6eeb795a01b79b08226c003e89865
    fingerprint: sha256:a618351863654efc379735ac3117d0193c690900f9c9832bdd094d4344af4c52
    observed: 2026-11-02T16:14:04Z
  - id: OBS-product-test-cb90047d10b04254e607a3a5
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-after-the-fix-the-2-issue-39-gate-tests-pass-the-full-suite
    aliases: OBS-product-test-11f42ac98b4076b73c7dbb9d
    claim: "After the fix: the 2 issue #39 gate tests pass, the full suite passes (198 passed, 0 failed), and the shipping reproduction is refused with refused:unlinked-failure naming the case and observation (exit 1, stage stays test)"
    result: passed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-linux-x86_64
    content: 82cdff22e5aa308438baaadfe1a43725c1e88cda9bdec1d93fc1930ce7cd7231
    fingerprint: sha256:7ec04bbdec8803da96034a585dbed0d1e351da9e7d3f112e083814e31d0c50eb
    observed: 2026-11-02T16:17:48Z
  - id: OBS-product-test-16dc837a3d844c63a41b6932
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-tc-cli-a-later-not-run-keeps-an-unlinked-failure-refused
    aliases: OBS-tc-cli-a-later-not-run-keeps-an-unlinked-failure-refused-9fe14fa45436050117427a42
    claim: "Ran cargo test --locked --test stages -- --exact a_later_not_run_keeps_an_unlinked_failure_refused for TC-cli-a-later-not-run-keeps-an-unlinked-failure-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_later_not_run_keeps_an_unlinked_failure_refused
    revision: 466594bf7ed98b332c701afe1cae4d0f70355aa1
    environment: local-linux-x86_64
    content: d21d5dae5bb5bdcc3d2ff2a9f6272b3eae5461f2d444534da2ab347bae47920d
    fingerprint: sha256:96d888abbc30f4d650a70422df50c6304b441f9eea11d8b78527c76981c61610
    case: TC-cli-a-later-not-run-keeps-an-unlinked-failure-refused
    case_digest: ddc5af0df1359588cb5a6bcdbd0b53e3e0cbbce4e1c874a1a70b1136f328301d
    observed: 2026-11-02T17:04:48Z
    duration_seconds: 0
environment: Linux x86_64, main 825a380, SpecSync 6.1.2, Fledge 1.8.0, hi 0.8.0, examples/shipping copy with SDD off
revision: 825a3805530456005e361dfa6d96d93eee52cd09
found_by: https://github.com/CorvidLabs/chough/issues/39
related: []
created: 2026-11-02T16:14:14Z
updated: 2026-11-02T17:05:02Z
---
# Leaving test ignores a referenced case's failed, unlinked observation

**What happened:** stage advance moves test to accept while the latest observation of a referenced case failed and no bug links it; chough next shows no warning
Actual: Exit 0; the stage becomes accept

**Steps:** In stage test, record a failed observation for a checkpoint case without adding a bug, then chough stage advance

**Expected:** The move is refused until the case passes or a bug links the failed observation

**Why it matters:** A known failure is left behind at test; FLOW-7 and LIFE-5 are not enforced

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T16:14:14Z · Reported (https://github.com/CorvidLabs/chough/issues/39); evidence: OBS-product-test-9e2d612534e636aa16dbc628
- 2026-11-02T16:19:05Z · status: open -> fix-pending-verification. Failing-first gate tests recorded; evidence: OBS-product-test-cb90047d10b04254e607a3a5
- 2026-11-02T16:19:05Z · status: fix-pending-verification -> verified. Fixed in src/stage.rs: refused:unlinked-failure; tests pass and the reproduction is refused; evidence: OBS-product-test-11f42ac98b4076b73c7dbb9d
- 2026-11-02T17:03:50Z · status: verified -> open. Reopened from 0xLeif's PR #43 review (discussion_r4445242804): a later not-run observation of the case clears the refusal, because the latest observation of any outcome is used. Regression test fails before the fix.; evidence: OBS-product-test-16dc837a3d844c63a41b6932
- 2026-11-02T17:05:02Z · status: open -> verified. Fixed: unlinked_failure ignores not-run observations when picking a case's latest result; the regression case passes locally.; evidence: OBS-tc-cli-a-later-not-run-keeps-an-unlinked-failure-refused-9fe14fa45436050117427a42
