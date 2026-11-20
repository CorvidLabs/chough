---
kind: evidence
id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-622b00
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:902432363527d7f153fce32f3e71fcbce53674a4b7f7c79067c8009b783e5ebe
criteria: [LIFE-10, LIFE-3]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:43Z. The raw log stays in the machine-local store.
