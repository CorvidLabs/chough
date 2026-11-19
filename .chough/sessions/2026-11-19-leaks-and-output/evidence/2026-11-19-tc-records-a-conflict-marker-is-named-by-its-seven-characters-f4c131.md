---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f4c131
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
duration_seconds: 1
revision: 908d3ace4545d270125d13706f6b012cdc276e21
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:64095815ffd1d69cce5d68e5abc320450829b188644b78aef976a3278006c694
criteria: [LIFE-26]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 908d3ace4545, observed 2026-11-19T05:11:35Z. The raw log stays in the machine-local store.
