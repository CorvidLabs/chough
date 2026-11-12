---
kind: bug
id: 2026-10-31-native-reads-collect-unbounded-output-and-have-no-deadline
aliases: [BUG-0011]
title: Native reads collect unbounded output and have no deadline
status: verified
classification: product-defect
scope: in-scope
impact: status, next and gates can hang or exhaust memory
criteria: [LIFE-7]
requirements: [REQ-cli-003, REQ-cli-006]
cases: []
evidence: [bug-0011-repro-1, bug-0011-fixed-1, tc-cli-0049-1793482745]
receipts:
  - id: 2026-10-31-chough-next-collected-400-mb-of-native-stdout-and-stderr-peak
    aliases: bug-0011-repro-1
    claim: chough next collected 400 MB of native stdout and stderr (peak 774 MiB) before reporting oversized, and waited on a native command that never exits
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:ed741d417559e25e79a0c81a4eb9e13d0b5508a85cc69ed2c2b95c453b798c98
    observed: 2026-10-31T20:44:40Z
  - id: 2026-10-31-tc-cli-0049-a4077d
    aliases: bug-0011-fixed-1
    claim: "Fixed binary from bbd9f04 with the recorded stand-in specsync scripts: 400 MB of stdout and stderr peaked at 10 MiB RSS and reported oversized with the exact 200000000-byte count; the never-exiting stand-in was reported timed-out after 2.65 s with CHOUGH_NATIVE_TIMEOUT_SECONDS=2 and after 120.30 s by default, with no surviving descendant; stage advance from plan refused it as native-unknown in 2.07 s."
    result: passed
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:c4a423a0f576e3c07a446bb874a2c0356ac0f66672119c95b588148e31c65f08
    case: TC-cli-0049
    case_digest: 79e2fe2c899710ae14a8801397cc57f21339fdbccfc74cb829b0268cd1eac967
    observed: 2026-10-31T21:45:01Z
  - id: 2026-10-31-tc-cli-0049
    aliases: tc-cli-0049-1793482745
    claim: "Ran cargo test --locked --test cli -- --exact native_reads_end_by_their_deadline_and_retain_bounded_output for TC-cli-0049: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact native_reads_end_by_their_deadline_and_retain_bounded_output
    revision: bbd9f041fb58f14ed1447420c183984c1a602446
    environment: local-macos-arm64
    fingerprint: sha256:e93f49a2553a0507f622b6be1bf0c7d4d52e9cf79635de3cf5ea6551e2cd39d6
    case: TC-cli-0049
    case_digest: 79e2fe2c899710ae14a8801397cc57f21339fdbccfc74cb829b0268cd1eac967
    observed: 2026-10-31T21:39:14Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding F, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:01Z
updated: 2026-10-31T21:45:11Z
---
# Native reads collect unbounded output and have no deadline

**What happened:** chough next held 400 MB of native output (774 MiB peak) and waited indefinitely on a stuck command
Actual: Command::output collects everything; there is no deadline

**Steps:** A native command that writes very large output or never exits
1. Put a stand-in specsync on PATH that writes 200 MB to each stream, or sleeps
2. chough next

**Expected:** Capture stops retaining at the parse bound while reading, stderr is bounded and drained, and a deadline reports the state as unknown

**Why it matters:** status, next and gates can hang or exhaust memory

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:01Z · Reported (Review finding F, reproduced 2026-10-31 against main f19823d); evidence: bug-0011-repro-1
- 2026-10-31T20:45:01Z · Added despite similar reports: BUG-0004 fixed parsing beyond the display bound; this is capture during reading
- 2026-10-31T21:45:11Z · status: open -> verified. Fixed in bbd9f04 (native reads through the bounded runner: 4 MiB + 1 stdout, 16 KiB stderr, 120 s deadline with CHOUGH_NATIVE_TIMEOUT_SECONDS, timed-out state); regression case TC-cli-0049 and the recorded stand-in rerun pass on macOS arm64.; evidence: bug-0011-fixed-1, tc-cli-0049-1793482745
