---
kind: bug
id: 2026-11-07-full-macos-lane-checkpoint-writer-lock-timeout
aliases: [BUG-full-macos-lane-checkpoint-writer-lock-timeout-ddac00a62f685f463c2b9f79]
title: Full macOS lane checkpoint writer lock timeout
status: wont-fix
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
updated: 2026-11-13T04:33:19Z
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
- 2026-11-12T17:09:52Z · status: open -> investigating. Not reproduced (t38b, 2026-11-12, diagnostics for Leif): the real test cli --exact checkpoint_writers_in_parallel_keep_every_update ran 24 times (4 copies x 6) under 30 CPU hogs, 4 F_FULLFSYNC writers and the stop and cases binaries, 0 failed at peak load 89.6; the same on the bug's own revision 3b9d008d, 24 runs, 0 failed at peak 83.8; the whole cli binary 15 loaded runs, 0 failed. A probe outside the harness (3 writers x 8 rounds) waited at most 60 ms unloaded, 165 ms with 60 hogs, 410 ms with 8 F_FULLFSYNC writers and 96 ms under an exec storm at load 97. Ruled out: a pipe deadlock (at most 2.9 KB of output per writer) and the first-exec check (the writers run only real git). Reaching the 30 s refusal needs one writer stuck for over 30 s, or two for over 15 s; that day's conditions (a full macOS verify beside a pinned Linux verification in Docker and a preservation audit) were not reproduced. If it recurs, the refusal names the lock holder's PID and the test prints each writer's lock wait; sample the holder then. Left open for Leif: a flake that does not reproduce is never closed to clear the tag · by claude-code (hc t382)
- 2026-11-12T17:11:33Z · status: investigating -> wont-fix. Closed as not reproducible by Leif's decision: "Close: can't reproduce (Recommended)" (option text: 'Closed as not reproducible, with the diagnostics and your words recorded. It reopens if it's ever seen again, and the 1.0 rule of two clean weeks still applies.'), coordinator chat (Claude Code, hc t370), 2026-11-12T17:11Z, relayed by claude-code-t370; recorded by claude-code-t382. Not fixed and not verified: t38b's loaded runs gave 0 failures in 24 runs at the head, 24 on the bug's revision 3b9d008d and 15 whole cli binary runs, and the longest lock wait probed was 410 ms (8 F_FULLFSYNC writers). It reopens if a checkpoint-busy refusal is seen again; the refusal now names the lock holder's PID · by claude-code (hc t382)
- 2026-11-13T04:33:19Z · Tracked by the flake window (DRIVE-19); an intermittent full-lane failure under load, recorded locally and closed wont-fix, no GitHub issue was filed · by claude-code (hc t38r)
