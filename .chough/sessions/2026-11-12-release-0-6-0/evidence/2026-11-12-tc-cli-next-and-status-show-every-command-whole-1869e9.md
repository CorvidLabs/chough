---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-show-every-command-whole-1869e9
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
duration_seconds: 3
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:791e28a9ce72521bceb5fb108a47a7c05bc07ccb599c2d7add284e2a6eb71212
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T21:03:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:37Z. The raw log stays in the machine-local store.
