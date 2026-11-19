---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ecf999
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
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:f07645f2cf84676aeb976fd7f5c8aa35a737d2bf9c0cf25a4e356cf588904dce
criteria: [LIFE-26]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:30Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:30Z. The raw log stays in the machine-local store.
