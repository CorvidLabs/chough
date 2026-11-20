---
kind: evidence
id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-5ac074
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-control-character-is-refused-at-write]
case_digest: [0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_control_character_is_refused_at_write_in_every_field
exit_code: 0
tests_run: 1
duration_seconds: 9
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:20d3a7f17dbe13bf38650c9b1760f348021d53360f32a69e67a3f661ef188ef0
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:31Z. The raw log stays in the machine-local store.
