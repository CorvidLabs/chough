---
kind: evidence
id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-d7da54
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-front-matter-conflict-is-named]
case_digest: [18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862]
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
duration_seconds: 1
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:6883bb4a4ad3e1c7853be05002e9015d8288f9e28bcdc00a1b6feaf9b86647ed
criteria: [DRIVE-17]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:50:32Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:50:32Z. The raw log stays in the machine-local store.
