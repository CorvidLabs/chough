---
kind: bug
id: 2026-11-03-checkpoint-capacity-refusal-publishes-orphan-gate-history
aliases: [BUG-checkpoint-capacity-refusal-publishes-orphan-gate-history-b53b7f8277c4ff818e09bde1]
title: Checkpoint capacity refusal publishes orphan gate history
status: verified
classification: product-defect
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-11, LIFE-7]
requirements: [REQ-cli-032]
cases: []
evidence:
  - OBS-product-test-2a7d7facc915bcf96b293641
  - OBS-product-test-d7d49ea83fedc7d95d30dba1
  - OBS-product-test-2f00d25a768981daf5bb6eb0
receipts:
  - id: 2026-11-03-failure-first-writer-regression-at-unchanged-production-source
    aliases: OBS-product-test-2a7d7facc915bcf96b293641
    claim: "Failure-first writer regression at unchanged production source: all three capacity-refusal assertions fail because rejected commands publish four orphan gate events. Exact formatted tests and source, command, binary hashes and raw output are frozen. Zero passed, three failed, thirty filtered. Exact-bound controls execute successfully before the final cursor overflow failure."
    result: failed
    revision: 9bd3d506b86cbb312a5a3e195697cd3138920039
    environment: local-macos-arm64
    content: b08fca69655e9791e5f80f89975cc8ec4c78f6621bda92eb93409f3fafdd733c
    fingerprint: sha256:e7315c898503f3aa83c85e9234d1a26f6ce0f3134aa4b18f6962191a07276a3f
    observed: 2026-11-03T06:37:07Z
  - id: 2026-11-03-all-three-writer-regressions-pass-with-identical-final-test
    aliases: OBS-product-test-d7d49ea83fedc7d95d30dba1
    claim: All three writer regressions pass with identical final test bytes in the Rust1.95 failure and repair pair. Exact-bound controls, selected and legacy history, clear/re-add ordering and all managed-record preservation pass. Formatter, cargo check and lint pass without suppressions.
    result: passed
    revision: 15a3b2ebd7df287615f6fbf171d547732a334d60
    environment: local-macos-arm64
    content: 28beaabe9b0e7fc0539b524e4609b503b66e7d3a37ce0ff009bf3aeba9872961
    fingerprint: sha256:e942486dd9365846b704e9528b004f7c78562bdc99f8b9f29dfc26ff05ce9a67
    observed: 2026-11-03T06:43:29Z
  - id: 2026-11-03-final-settled-source-qualification-all252-rust-tests-across12
    aliases: OBS-product-test-2f00d25a768981daf5bb6eb0
    claim: "Final settled-source qualification: all252 Rust tests across12 suites pass with zero failed, ignored or filtered; all9 Fledge steps pass; all18 archive fixtures pass. Three formal <redacted:host-name> report cases also pass at the same source. The earlier log-path-induced stale audit and original failure-first logs are preserved separately."
    result: passed
    revision: 98a86bc14609c1c4cb68699b41ca9fa24665943b
    environment: local-macos-arm64
    content: 927abca849c738bbbd1dbc1f4065676128a1532164dc1fd295cb40ecc49f394d
    fingerprint: sha256:ef485454f54abf2447413c3403c82259160468822bfbe01cdeeddf07b188794c
    observed: 2026-11-03T06:52:41Z
fixture: .chough/artifacts/pr51-writer-atomicity/before/
environment: local-macos-arm64
revision: 9bd3d506b86cbb312a5a3e195697cd3138920039
found_by: PR51 discussion_r4451099601, confirmed by root failure-first regression
related: []
created: 2026-11-03T06:37:23Z
updated: 2026-11-03T06:52:42Z
---
# Checkpoint capacity refusal publishes orphan gate history

**What happened:** A rejected capacity-overflow checkpoint command creates immutable gate events before candidate validation.
Actual: All three stable regressions fail with four newly written orphan event files in each first failing empty-head fixture.

**Steps:** Writer update with observations400to401 or serialized bytes131073 and clear/open gate changes.
1. Run fledge run test -- --locked --test cli checkpoint_writer_ -- --nocapture against frozen unchanged production source.

**Expected:** Named refusal leaves every managed record path and byte identical.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-03T06:37:23Z · Reported (PR51 discussion_r4451099601, confirmed by root failure-first regression); evidence: OBS-product-test-2a7d7facc915bcf96b293641
- 2026-11-03T06:43:30Z · status: open -> verified. Final identical test bytes genuinely fail before repair and pass after it under Rust1.95. Three full writer regressions and fmt/check/lint pass; all earlier proof is unchanged.; evidence: OBS-product-test-d7d49ea83fedc7d95d30dba1
- 2026-11-03T06:52:42Z · Fresh full252-test pass after settled source tasks and18 real archive fixtures; writer regressions pass within the full suite, zero skipped or filtered. Historical failure and focused pair stay unchanged.; evidence: OBS-product-test-2f00d25a768981daf5bb6eb0
