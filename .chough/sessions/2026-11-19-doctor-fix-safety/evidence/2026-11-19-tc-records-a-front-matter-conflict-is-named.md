---
kind: evidence
id: 2026-11-19-tc-records-a-front-matter-conflict-is-named
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
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:45629ef0b43e83f317f0da430aaf1ba3a6d0eb39b5fd16adf4f857cca0e44e91
criteria: [DRIVE-17]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:32:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:32:14Z. The raw log stays in the machine-local store.
