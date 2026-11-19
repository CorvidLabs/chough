---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ad0790
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
revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
environment: local-macos-arm64
fingerprint: sha256:21bbac3fea453ead7927d6b7c3e4622b6894d2c806373d1552bb2a3f5515e3f1
criteria: [LIFE-26]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:44:29Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 402a3a5d4ae9, observed 2026-11-19T02:44:29Z. The raw log stays in the machine-local store.
