---
kind: evidence
id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-55f86d
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-no-command-prints-a-stored-control-character-raw]
case_digest: [c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - no_command_prints_a_stored_control_character_raw
exit_code: 0
tests_run: 1
duration_seconds: 10
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:02058d7c2958a362d0f5b9c4eaa99c10a776f4cf45eb39f92ef692431e972b0c
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:46Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:46Z. The raw log stays in the machine-local store.
