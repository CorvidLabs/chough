---
kind: evidence
id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-5d2141
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-front-matter-conflict-is-named]
case_digest: [c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_front_matter_conflict_is_named_and_resolved_not_shelved
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:c6423a1a85a6989add674a7a2b5de93257653d683dc40ea0d498e52545ae19e8
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:06Z. The raw log stays in the machine-local store.
