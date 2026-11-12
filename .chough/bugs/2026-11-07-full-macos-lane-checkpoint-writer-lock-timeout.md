---
kind: bug
id: 2026-11-07-full-macos-lane-checkpoint-writer-lock-timeout
aliases: [BUG-full-macos-lane-checkpoint-writer-lock-timeout-ddac00a62f685f463c2b9f79]
title: Full macOS lane checkpoint writer lock timeout
status: open
classification: unclassified
scope: out-of-scope
impact: Not yet assessed.
criteria: []
requirements: []
cases: []
evidence: [OBS-product-test-80b95a49cc2ba0705ed1b2fb, OBS-product-test-4f7d63e468288e44cfd85c88]
receipts:
  - id: 2026-11-07-initial-full-verify-at
    aliases: OBS-product-test-80b95a49cc2ba0705ed1b2fb
    claim: "Initial full verify at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9 failed step3: checkpoint_writers_in_parallel_keep_every_update hit checkpoint-busy after30s; Fledge exit1/cargo101. Cause unverified; raw receipt retained."
    result: failed
    revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
    environment: local-macos-arm64
    content: 92f2383e07014789609a16a6d4944473a925a62695daa8440da9f3fe33ac06c1
    fingerprint: sha256:902c37bc572b3bc586471b857b5cc781ed8256a1b9a6a688b254703c216e61cb
    observed: 2026-11-07T17:19:40Z
  - id: 2026-11-07-sequential-full-verify-at
    aliases: OBS-product-test-4f7d63e468288e44cfd85c88
    claim: "Sequential full verify at 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9: all10steps exit0, 596 product tests passed. Isolated prior failed checkpoint test passed exit0/1test in1.20s with unchanged timeout/assertions. Initial failure retained."
    result: passed
    revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
    environment: local-macos-arm64
    content: 92f2383e07014789609a16a6d4944473a925a62695daa8440da9f3fe33ac06c1
    fingerprint: sha256:97da695b9691372e36913cccd34f7b0fde2a3e3a6fe8e0ede8a823a79adc1f84
    observed: 2026-11-07T17:19:42Z
environment: local-macos-arm64
revision: 3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9
found_by: Corrective PR197 qualification; tests/cli.rs:2072 outside the two approved P1 behaviors
related: []
created: 2026-11-07T17:19:46Z
updated: 2026-11-07T17:19:46Z
---
# Full macOS lane checkpoint writer lock timeout

**What happened:** Parallel checkpoint writer child refused checkpoint-busy after30seconds in initial full lane
Actual: Initial full lane failed exit1/cargo101; identical isolated test and sequential full lane later passed without changing inputs, assertions or timeout. Root cause unverified.

**Steps:** Run full macOS verify concurrently with pinned Linux verification and read-only preservation audit

**Expected:** Existing checkpoint writer test completes without lost updates or timeout

**Why it matters:** Not yet assessed.

**Fix:** Not fixed yet.

## Log

- 2026-11-07T17:19:46Z · Reported (Corrective PR197 qualification; tests/cli.rs:2072 outside the two approved P1 behaviors); evidence: OBS-product-test-80b95a49cc2ba0705ed1b2fb, OBS-product-test-4f7d63e468288e44cfd85c88
