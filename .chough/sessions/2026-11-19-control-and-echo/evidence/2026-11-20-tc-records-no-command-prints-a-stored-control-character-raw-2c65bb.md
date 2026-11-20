---
kind: evidence
id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-2c65bb
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
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:b6a29b0b76db33a2fa33af9264d0cec2cf7175c3e0c5ca3028babf598f9c10b5
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:38:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:38:47Z. The raw log stays in the machine-local store.
