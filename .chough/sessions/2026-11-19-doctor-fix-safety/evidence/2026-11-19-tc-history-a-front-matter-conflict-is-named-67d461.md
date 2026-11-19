---
kind: evidence
id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-67d461
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
revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
environment: local-macos-arm64
fingerprint: sha256:6c34efeda417ed93a9e6aa1f10bb8fefcefbccf0cc698a8668865395f63dcdc0
criteria: [DRIVE-17]
requirements: [REQ-records-001, REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:38:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f09cf6ba0f1e, observed 2026-11-19T05:38:25Z. The raw log stays in the machine-local store.
