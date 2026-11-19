---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-dab931
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
duration_seconds: 0
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:110c69671dd2eb84adee5f9dcb069c6a0f4bb6fcf15b42add8dba4ba2a6db2b6
criteria: [LIFE-26]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:04Z. The raw log stays in the machine-local store.
