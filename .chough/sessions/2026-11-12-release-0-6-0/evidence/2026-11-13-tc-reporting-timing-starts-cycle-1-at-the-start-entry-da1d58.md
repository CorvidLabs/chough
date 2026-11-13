---
kind: evidence
id: 2026-11-13-tc-reporting-timing-starts-cycle-1-at-the-start-entry-da1d58
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:0dd257de2db3ccf697e813493e09ded9d858c16686f47077dfef3f34b67e7145
criteria: [LIFE-7, LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T00:50:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:50:49Z. The raw log stays in the machine-local store.
