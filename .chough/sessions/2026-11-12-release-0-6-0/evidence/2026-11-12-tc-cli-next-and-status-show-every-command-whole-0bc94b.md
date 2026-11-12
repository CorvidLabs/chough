---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-show-every-command-whole-0bc94b
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:dbad516b4f4320709f2c5758dd1a4aba9af389cd8c7d65cbbab084638c1d903b
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T23:23:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:23:22Z. The raw log stays in the machine-local store.
