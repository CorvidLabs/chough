---
kind: bug
id: 2026-10-31-stop-requested-during-a-validation-interval-still-allows
aliases: [BUG-0009]
title: Stop requested during a validation interval still allows another attempt
status: verified
classification: product-defect
scope: in-scope
impact: Repeated runs continue after the human asked them to stop
criteria: [FLOW-9]
requirements: [REQ-cli-011]
cases: []
evidence: [bug-0009-repro-1, bug-0009-fixed-1, tc-cli-0048-1793482740]
receipts:
  - id: 2026-10-31-a-stop-requested-1-5-s-into-a-4-s-validation-interval-still
    aliases: bug-0009-repro-1
    claim: A stop requested 1.5 s into a 4 s validation interval still allowed a second attempt (completed 2 of 3)
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:5dbdd9fcb9286ce4068d0456359fb12d8aa305305c4bfcb9238ca68ebcba8b51
    observed: 2026-10-31T20:44:39Z
  - id: 2026-10-31-tc-cli-0048-2c6ded
    aliases: bug-0009-fixed-1
    claim: "Fixed binary from bbd9f04 on the recorded reproduction (disposable copy of examples/shipping): a stop requested 1.5 s into a 4 s validation interval ended the run as stopped after 1 of 3 attempts in 2 s; no attempt started after the stop."
    result: passed
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:78646e12853179a3a7a1cd9e76200e059ab9d4f357f803cec55c6e4ba6925b8f
    case: TC-cli-0048
    case_digest: bd6a056713ccc06f0663e0a43059237ada37e17a36225b69ad6a2b8444903c9c
    observed: 2026-10-31T21:45:00Z
  - id: 2026-10-31-tc-cli-0048
    aliases: tc-cli-0048-1793482740
    claim: "Ran cargo test --locked --test optional -- --exact repeated_runs_halt_promptly_for_a_stop_or_pause_during_the_interval for TC-cli-0048: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact repeated_runs_halt_promptly_for_a_stop_or_pause_during_the_interval
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:6a9f4585a10ec9e463fd5bafd13cd5806640977977175792657d6f9de9a8342c
    case: TC-cli-0048
    case_digest: bd6a056713ccc06f0663e0a43059237ada37e17a36225b69ad6a2b8444903c9c
    observed: 2026-10-31T21:39:05Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding D, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:00Z
updated: 2026-10-31T21:45:11Z
---
# Stop requested during a validation interval still allows another attempt

**What happened:** validate run --times 3 --interval 4 completed 2 attempts after a stop at 1.5 s
Actual: The stop file is checked before the sleep only

**Steps:** chough validate stop while a repeated run sleeps between attempts
1. Grant repeat-validation consent
2. validate run <case> --times 3 --interval 4
3. validate stop during the first interval

**Expected:** No further attempt starts once a stop or pause exists, including after the wait

**Why it matters:** Repeated runs continue after the human asked them to stop

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:00Z · Reported (Review finding D, reproduced 2026-10-31 against main f19823d); evidence: bug-0009-repro-1
- 2026-10-31T20:45:00Z · Added despite similar reports: Stop timing during the interval, not stop between attempts
- 2026-10-31T21:45:11Z · status: open -> verified. Fixed in bbd9f04 (stop and pause checked before every attempt, interval waited in 200 ms slices); regression case TC-cli-0048 and the recorded reproduction rerun pass on macOS arm64.; evidence: bug-0009-fixed-1, tc-cli-0048-1793482740
