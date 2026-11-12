---
kind: evidence
id: 2026-11-12-tc-reporting-timing-starts-cycle-1-at-the-start-entry
aliases: []
claim: "Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: failed (exit 101)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-reporting-timing-starts-cycle-1-at-the-start-entry]
case_digest: [4d0469e15fa660ca796b7df1cb6dd1978f1d168b1e88c748364b3c1e2a942ace]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - timing
  - "--"
  - timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once
  - init_and_session_new_write_a_start_entry_that_timing_reads
exit_code: 101
duration_seconds: 1
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:c94c8fb04b041308394192ebd36334fb0185122565b6c16dbba2c2f4a421629b
criteria: [LIFE-7, LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - "expected output missing: \"test result: ok. 2 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T22:56:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:56:16Z. The raw log stays in the machine-local store.

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
-      Running tests/timing.rs (target/debug/deps/timing-8ea5e96ca213ebda)
- error: test failed, to rerun pass `--test timing`
