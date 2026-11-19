---
kind: evidence
id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-610b0b
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
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:62fa842b199975c78c446dd9394e3ed874ac8f6945d79ff83a4beec227164034
criteria: [DRIVE-17]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:58Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:58Z. The raw log stays in the machine-local store.
