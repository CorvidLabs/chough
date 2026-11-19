---
kind: evidence
id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-e19237
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
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:64046a05f8105165f1a8d81c96e65b6e897298aacb94dcccd1bbfdaf10992fc3
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:52Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:52Z. The raw log stays in the machine-local store.
