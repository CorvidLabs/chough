---
kind: evidence
id: 2026-11-13-tc-cli-next-and-status-show-every-command-whole-b0f465
aliases: []
claim: "Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: passed (exit 0)."
result: passed
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
exit_code: 0
duration_seconds: 4
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:a884f96dd33cabd6448541430f4a09ee55c2f05a3376a41165edb0e5c9e5d7a6
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T00:45:48Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:45:48Z. The raw log stays in the machine-local store.
