---
kind: evidence
id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-5847df
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
duration_seconds: 7
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:e08d11d3f1cbc199484c9bc2ac382a419d9a42b71dcdc7c475262b611c3304e8
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:01Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:01Z. The raw log stays in the machine-local store.
