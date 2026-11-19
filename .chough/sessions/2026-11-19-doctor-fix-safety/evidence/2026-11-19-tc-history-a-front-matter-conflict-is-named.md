---
kind: evidence
id: 2026-11-19-tc-history-a-front-matter-conflict-is-named
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-front-matter-conflict-is-named]
case_digest: [a52a4bd47d867f61e33e4c18ef7edaa7111334bd8785dcb18063c54cde3feb8e]
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
duration_seconds: 3
revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
environment: local-macos-arm64
fingerprint: sha256:d287131568c407e7e7ad7af09c1c61535923ba1a2bbf16ff5ffa177066930a37
criteria: [DRIVE-17]
requirements: [REQ-records-001, REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:34:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 73cf8c7aaca2, observed 2026-11-19T05:34:20Z. The raw log stays in the machine-local store.
