---
kind: evidence
id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-192171
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:76940c2358314e338e172cf10744f798b98d3dc148e89ffe7324cd93b07e4ef3
criteria: [LIFE-10, LIFE-3]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:45Z. The raw log stays in the machine-local store.
