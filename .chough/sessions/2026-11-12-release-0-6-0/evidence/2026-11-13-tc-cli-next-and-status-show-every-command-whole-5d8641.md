---
kind: evidence
id: 2026-11-13-tc-cli-next-and-status-show-every-command-whole-5d8641
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:c5cf778432df8faaf7de536f976d54d912e105e1434e5d79a771654663f14cd7
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T01:12:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- next_and_status_show_every_command_whole_for_a_long_change_id status_gives_the_next_valid_action_whole both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-next-and-status-show-every-command-whole: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:12:12Z. The raw log stays in the machine-local store.
