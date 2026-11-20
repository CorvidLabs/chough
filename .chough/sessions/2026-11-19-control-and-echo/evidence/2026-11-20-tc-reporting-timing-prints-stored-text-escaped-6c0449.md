---
kind: evidence
id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6c0449
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:82c320fd5d772915121e52ee240c6b3e6ba024e1cddb8c67bc07a9e4341b4a1f
criteria: [LIFE-10, LIFE-3]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:38Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:38Z. The raw log stays in the machine-local store.
