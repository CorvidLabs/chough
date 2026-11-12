---
kind: bug
id: 2026-11-03-observation-overflow-refusal-omits-its-numeric-capacity-bound
aliases:
  - BUG-observation-overflow-refusal-omits-its-numeric-capacity-bound-ed2d02c311446ef92cc449c5
title: Observation overflow refusal omits its numeric capacity bound
status: verified
classification: product-defect
scope: blocking
impact: The named capacity requirement is not met and the operator cannot see which numeric limit applies.
criteria: [LIFE-11]
requirements: [REQ-cli-032]
cases: []
evidence:
  - OBS-product-test-f070b3954a220492de975d64
  - OBS-product-test-19fbf7ce1563d5f4dd78801d
  - OBS-product-test-e613f31740235919b76505d7
receipts:
  - id: 2026-11-03-failure-first-the-unchanged401-observation-fixture-fails-its
    aliases: OBS-product-test-f070b3954a220492de975d64
    claim: "Failure-first: the unchanged401-observation fixture fails its added numeric 'limit 400' assertion because the actual diagnostic is 'too many observation'. Focused Fledge wrapper exits1/Cargo101,0passed,1failed,28filtered. Production diagnostic source is unchanged; exact inputs and output are frozen."
    result: failed
    revision: dbb66d979e2c34e41926401641d58971c0d65f65
    environment: local-macos-arm64
    content: 65b9e4afd694c8a5fae424aec021290059b6249bbb7c4a5cd0422a11f629e21e
    fingerprint: sha256:b9f2b559adb0d166edcd57187a0a29163c35e587f1c384f2d92034886438ecf3
    observed: 2026-11-03T05:46:47Z
  - id: 2026-11-03-paired-diagnostic-regression-the-same-unchanged-assertion-now
    aliases: OBS-product-test-19fbf7ce1563d5f4dd78801d
    claim: "Paired diagnostic regression: the SAME unchanged assertion now passes after the sole ensure_ids message repair. Actual refusal names limit400. Focused Fledge/Cargo exits0,1passed,0failed/ignored,28filtered; fmt/check/lint pass. Limits, path validation and atomic behavior are unchanged."
    result: passed
    revision: 6822becf356d130bb3a2b6ac41c07eef8448eaba
    environment: local-macos-arm64
    content: f3c5c91b415e68a74818c19004716aadd86b3483b5bf638f0f0baf8bc40dcf8b
    fingerprint: sha256:c97065b518c01aa511f4248613f1917d11e7f9880128764761492891257651a7
    observed: 2026-11-03T05:50:22Z
  - id: 2026-11-03-final-integrated-capacity-source-passes-the-full-nine-step
    aliases: OBS-product-test-e613f31740235919b76505d7
    claim: Final integrated capacity source passes the full nine-step Fledge lane and every Rust test, including unchanged exact/over byte and400/401pointer boundaries, other-record/list isolation, imported CLI/identity/report regressions and the named limit400assertion. Three supported actualMac report cases also pass. Actual input/output and artifact identities are retained.
    result: passed
    revision: 4ec128c9ddb838c7aede89bbd90dac94836b0299
    environment: local-macos-arm64
    content: 7d22fd2c5f3ea725cc8d093a7024881b7564279b0413ed75922da777d433f5a2
    fingerprint: sha256:edc39bb75f65b4d58e859c6ea0cd08ca301dab4e970197459ff53649f331c259
    observed: 2026-11-03T06:09:00Z
environment: local-macos-arm64
revision: dbb66d979e2c34e41926401641d58971c0d65f65
found_by: root independent review and genuine unchanged failure-first regression at current working inputs
related: []
created: 2026-11-03T05:46:48Z
updated: 2026-11-03T06:09:00Z
---
# Observation overflow refusal omits its numeric capacity bound

**What happened:** 401observation pointers refuse without naming the400limit required byREQ-cli-032
Actual: The actual refusal is only Error: too many observation and the focused assertion fails.

**Steps:** Run the existing observation pointer boundary test with an unchanged added limit400diagnostic assertion

**Expected:** The refusal identifies the observation list and numeric400limit while preserving the checkpoint bytes.

**Why it matters:** The named capacity requirement is not met and the operator cannot see which numeric limit applies.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-03T05:46:48Z · Reported (root independent review and genuine unchanged failure-first regression at current working inputs); evidence: OBS-product-test-f070b3954a220492de975d64
- 2026-11-03T05:50:23Z · status: open -> verified. The preserved failed assertion is unchanged; a later genuine focused pass follows the sole numeric diagnostic repair. Final integrated full-suite and hosted qualification remain to run.; evidence: OBS-product-test-19fbf7ce1563d5f4dd78801d
- 2026-11-03T06:09:00Z · Fresh final integrated full-suite pass after accepted PR50 import confirms the preserved unchanged diagnostic regression and capacity isolation; source/environment/artifact identities are current.; evidence: OBS-product-test-e613f31740235919b76505d7
