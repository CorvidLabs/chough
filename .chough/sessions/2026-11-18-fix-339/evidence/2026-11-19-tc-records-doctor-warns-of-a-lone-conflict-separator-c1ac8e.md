---
kind: evidence
id: 2026-11-19-tc-records-doctor-warns-of-a-lone-conflict-separator-c1ac8e
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-warns-of-a-lone-conflict-separator]
case_digest: [e69b8fa61b10bc09762f5326d80b8660eed248aa91c8256d45ac7f8df8881267]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_warns_of_a_lone_conflict_separator_without_failing
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 08801812d7fe82d42f54451eb25087df5ddaeec6
content: 2b860426c9c1f962e92c5eaef48d4cf1ed31a739e1fc4da29359c8cb5baba200
environment: local-macos-arm64
fingerprint: sha256:b0830ec9d1457b4ae482f3a2d4aaffde77875cfa0c19c1c14e67607a7a9abc4a
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:41:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08801812d7fe, observed 2026-11-19T00:41:20Z. The raw log stays in the machine-local store.
