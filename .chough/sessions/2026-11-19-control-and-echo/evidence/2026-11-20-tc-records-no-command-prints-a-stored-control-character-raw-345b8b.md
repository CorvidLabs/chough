---
kind: evidence
id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-345b8b
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:964ee6d0e203c6390c26a3ce1c3c3161e828b4e8fb729a83f21b53a0ad2eede9
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:22Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:22Z. The raw log stays in the machine-local store.
