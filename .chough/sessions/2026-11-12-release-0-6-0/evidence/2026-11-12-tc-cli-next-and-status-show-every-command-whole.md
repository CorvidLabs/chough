---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-show-every-command-whole
aliases: []
claim: "Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: failed (exit 101)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-and-status-show-every-command-whole]
case_digest: [ec0371fa11832861715a42f711362537aaa99fe50cb04a2fbfd2cac21a466a5e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - next_and_status_show_every_command_whole_for_a_long_change_id
  - status_gives_the_next_valid_action_whole
  - both_forms_stay_within_the_cap_with_totals_for_large_records
exit_code: 101
duration_seconds: 5
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:15d5e753f437fd60771cc91a1b1540d91d13f149a17948c32ba5774e0ca94732
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"test result: ok. 3 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T21:01:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:53Z. The raw log stays in the machine-local store.

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
-      Running tests/next.rs (target/debug/deps/next-9aae4320eafb1bee)
- error: test failed, to rerun pass `--test next`
