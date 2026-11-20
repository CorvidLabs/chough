---
kind: evidence
id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6d845e
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
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:7f73730557ec6f3f9a67c080e7ac22b1c9dc865b6713a1937af00c41e8b32a0f
criteria: [LIFE-10, LIFE-3]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:34:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:34:33Z. The raw log stays in the machine-local store.
