---
kind: bug
id: 2026-11-02-timing-native-budget-regression-sometimes-records-no-initial
aliases:
  - BUG-timing-native-budget-regression-sometimes-records-no-initial-stub-call-1d6cba28a1515f6745b80fea
title: Timing native-budget regression sometimes records no initial stub call on MSRV
status: verified
classification: unclassified
scope: in-scope
impact: Unresolved qualification reliability concern; no timing code changed in the workflow documentation PR.
criteria: [LIFE-10]
requirements: [REQ-cli-024]
cases: []
evidence:
  - OBS-product-test-c640523df825794a3f2f64e3
  - OBS-tc-cli-agent-native-workflow-upgrade-c87dc40990d5b05c8f45e17d
  - 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-timing-native-budget-flake-tests-timing-rs-a
  - 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-timing-native-budget-flake-tests-timing-rs
receipts:
  - id: 2026-11-02-initial-final-docs-msrv-run-failed-in-unchanged-timing-native
    aliases: OBS-product-test-c640523df825794a3f2f64e3
    claim: "Initial final-docs MSRV run failed in unchanged timing native-budget regression: no stub call recorded within the two-second whole-report budget; cause not yet established."
    result: failed
    revision: 9c41e95949adcb1009c986f97db50faa593d81b2
    environment: local-macos-arm64-rust1.88
    content: 983bb9f01666418f7e9163725d5ea41c30d271448c4b62b07d35b16604616a60
    fingerprint: sha256:4a5c9f45852786e0237b6f3dbba1be09a594c6db406d2f5c074a795473a72239
    observed: 2026-11-02T18:12:19Z
  - id: 2026-11-02-tc-cli-agent-native-workflow-upgrade-d1ea70
    aliases: OBS-tc-cli-agent-native-workflow-upgrade-c87dc40990d5b05c8f45e17d
    claim: The full unchanged Rust1.88 rerun passes all198tests across11suites, zero failures/ignored; initial timing failure and successful isolated rerun are retained, cause unconfirmed.
    result: passed
    revision: 9c41e95949adcb1009c986f97db50faa593d81b2
    environment: local-macos-arm64-rust1.88
    content: 983bb9f01666418f7e9163725d5ea41c30d271448c4b62b07d35b16604616a60
    fingerprint: sha256:b73db9c4e71fe0129c3410fe98657126afe64478971eb0c12196deffebe560fd
    case: TC-cli-agent-native-workflow-upgrade
    case_digest: df58494ed355c0121e0e12514d3a6b539bec2588121efa437b06a3f2bc9238fd
    observed: 2026-11-02T18:16:30Z
environment: local-macos-arm64-rust1.88
revision: 9c41e95949adcb1009c986f97db50faa593d81b2
found_by: Observed PR44 final user-docs MSRV qualification
related: []
created: 2026-11-02T18:12:20Z
updated: 2026-11-12T16:54:35Z
---
# Timing native-budget regression sometimes records no initial stub call on MSRV

**What happened:** The unchanged timeout test expected cur-change in the stub-call log but found no calls.
Actual: The test failed at timing.rs1475 with an empty call list.

**Steps:** Full Rust1.88 qualification with concurrent independent local builds and tests.

**Expected:** The current native read starts within the two-second fixture budget, then times out; later reads are not started.

**Why it matters:** Unresolved qualification reliability concern; no timing code changed in the workflow documentation PR.

**Fix:** Not fixed yet.

## Log

- 2026-11-02T18:12:20Z · Reported (Observed PR44 final user-docs MSRV qualification); evidence: OBS-product-test-c640523df825794a3f2f64e3
- 2026-11-02T18:16:30Z · The isolated unchanged timeout test and full unchanged MSRV suite subsequently passed. No budget or assertion changed. Cause remains unconfirmed; keep this reliability concern open and out of the workflow-docs repair scope.; evidence: OBS-tc-cli-agent-native-workflow-upgrade-c87dc40990d5b05c8f45e17d
- 2026-11-12T16:54:35Z · status: open -> verified; scope: out-of-scope -> in-scope; evidence: 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-timing-native-budget-flake-tests-timing-rs-a, 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-timing-native-budget-flake-tests-timing-rs. Root cause (t37f, measured): macOS assesses a newly written executable on its first exec, and under load that runs seconds after spawn() returns (median 2,267 ms, 191 of 200 at 2 s or more); at 9c41e959 the 2 s-budget read was the fresh SpecSync stand-in's first exec, so Chough rightly stopped it before its first line logged the call. Fix: the stand-in is warmed outside every budget (t37f 7210d0e7, integrated as b6fcd0e9, and t38b's install-time warm-up 1888f4f5); no budget or assertion changed. Rerun by t382 with t37f's scripts: red at 9c41e959 3/20 passed, green at b6fcd0e9 20/20 · by claude-code (hc t382)
