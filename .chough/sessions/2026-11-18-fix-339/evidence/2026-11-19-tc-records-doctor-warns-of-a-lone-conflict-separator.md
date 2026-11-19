---
kind: evidence
id: 2026-11-19-tc-records-doctor-warns-of-a-lone-conflict-separator
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
revision: e72be7de72efed1c9917e9d18329bb4388c3c29a
content: ef0e3be4bc113f74006c5b98c137f39a04a285bdc4e2cec0d9126fc4328b6a36
environment: local-macos-arm64
fingerprint: sha256:25cea8ea1aa82a22790d829d74061e038895be5423933d092959f8d2a73c8614
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:21:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e72be7de72ef, observed 2026-11-19T00:21:34Z. The raw log stays in the machine-local store.
