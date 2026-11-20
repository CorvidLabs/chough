---
kind: evidence
id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-26365a
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:45dce0201661bac9ebeb2eebf951f4b528ebe1c6019bd327a09f8bc3a7cfbbd2
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:14Z. The raw log stays in the machine-local store.
