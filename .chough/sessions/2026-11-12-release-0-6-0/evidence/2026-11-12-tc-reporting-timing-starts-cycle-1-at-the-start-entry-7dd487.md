---
kind: evidence
id: 2026-11-12-tc-reporting-timing-starts-cycle-1-at-the-start-entry-7dd487
aliases: []
claim: "Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: passed (exit 0)."
result: passed
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
exit_code: 0
duration_seconds: 0
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:89c74787ac27a9260484e02cb2525b708e067d5705ab37642587d1bda37d6e34
criteria: [LIFE-7, LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T23:26:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:26:18Z. The raw log stays in the machine-local store.
