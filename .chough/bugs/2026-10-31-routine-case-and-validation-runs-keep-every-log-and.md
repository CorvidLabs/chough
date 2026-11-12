---
kind: bug
id: 2026-10-31-routine-case-and-validation-runs-keep-every-log-and
aliases: [BUG-0012]
title: Routine case and validation runs keep every log and observation with no retention
status: verified
classification: product-defect
scope: in-scope
impact: Raw logs accumulate without bound
criteria: [LIFE-8]
requirements: [REQ-cli-018]
cases: []
evidence: [bug-0012-repro-1, tc-cli-0053-1793488010]
receipts:
  - id: 2026-10-31-at-d761ff2-the-eight-req-cli-018-retention-tests-failed-0
    aliases: bug-0012-repro-1
    claim: "At d761ff2 the eight REQ-cli-018 retention tests failed (0 passed, 8 failed): evidence prune, the retention consent and the retention preference did not exist"
    result: failed
    revision: d4ca8fdf153bc022a7a7698b6027f14ef75cb810
    environment: local-macos-arm64
    fingerprint: sha256:53ada778843ad555be512c7df5bbc7e0b61a25736d310fe51d349a3306990be7
    observed: 2026-10-31T23:06:36Z
  - id: 2026-10-31-tc-cli-0053
    aliases: tc-cli-0053-1793488010
    claim: "Ran cargo test --locked --test evidence -- --exact apply_needs_consent_or_the_standing_preference_and_appends_the_ledger for TC-cli-0053: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test evidence -- --exact apply_needs_consent_or_the_standing_preference_and_appends_the_ledger
    revision: 30b74f8cb0f4daae07aa61fef59c38ea2e39a0c2
    environment: local-macos-arm64
    fingerprint: sha256:50e718c367328985651fb6ae76f60f5de64b95e22cf50d354a4e66265647b825
    case: TC-cli-0053
    case_digest: f1b6202da3747f39d0c59b7d975f97f9746bc41f1b94631a9c1eecf16abf211b
    observed: 2026-10-31T23:06:52Z
environment: local-macos-arm64
revision: d4ca8fdf153bc022a7a7698b6027f14ef75cb810
found_by: Failing-first tests of the approved release candidate, 2026-10-31
related: []
created: 2026-10-31T23:06:36Z
updated: 2026-10-31T23:07:21Z
---
# Routine case and validation runs keep every log and observation with no retention

**What happened:** No preview or policy removes superseded routine runs; the REQ-cli-018 tests fail 8 of 8
Actual: evidence prune does not exist; nothing is ever pruned

**Steps:** Repeated cases run or validate run --times N
1. cargo test --test evidence at d761ff2

**Expected:** Reference-aware preview, policy-gated pruning, a ledger and current/historical/pruned/unavailable labels

**Why it matters:** Raw logs accumulate without bound

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T23:06:36Z · Reported (Failing-first tests of the approved release candidate, 2026-10-31); evidence: bug-0012-repro-1
- 2026-10-31T23:07:21Z · status: open -> verified. Fixed by the release-candidate retention implementation; TC-cli-0050 to TC-cli-0061 passed at the committed tip (observation tc-cli-0053-1793488010 for the apply/ledger test).; evidence: tc-cli-0053-1793488010
