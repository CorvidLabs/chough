---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f33dcf
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-conflict-marker-is-named-by-its-seven-characters]
case_digest: [49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_conflict_marker_is_named_by_its_seven_characters
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:c41d6a13aff1696ba2da50464a8863125370073c93973d7c4f099c61447757a9
criteria: [LIFE-26]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:24Z. The raw log stays in the machine-local store.
