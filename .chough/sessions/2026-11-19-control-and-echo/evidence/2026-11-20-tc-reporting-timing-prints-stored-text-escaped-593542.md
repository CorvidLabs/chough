---
kind: evidence
id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-593542
aliases: []
claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-reporting-timing-prints-stored-text-escaped]
case_digest: [9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - timing
  - "--"
  - "--exact"
  - timing_prints_a_stored_control_character_escaped
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:a85bfd36c983d773b2bb8621b2eaad654cf7a118ce6a04d43910ec837a6ab3c2
criteria: [LIFE-10, LIFE-3]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:27:03Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:27:03Z. The raw log stays in the machine-local store.
