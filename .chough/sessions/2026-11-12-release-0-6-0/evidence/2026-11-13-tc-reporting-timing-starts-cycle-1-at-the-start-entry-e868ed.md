---
kind: evidence
id: 2026-11-13-tc-reporting-timing-starts-cycle-1-at-the-start-entry-e868ed
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:9c288206e3705a8739fd64f379df6ffd3f9a7e8cefc3a7639b65cd28c32952e3
criteria: [LIFE-7, LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T01:18:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- timing_starts_cycle_1_at_the_start_entry_and_says_elapsed_wall_clock_once init_and_session_new_write_a_start_entry_that_timing_reads for TC-reporting-timing-starts-cycle-1-at-the-start-entry: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:18:25Z. The raw log stays in the machine-local store.
