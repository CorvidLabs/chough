---
kind: evidence
id: 2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator
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
revision: af8ca8901b2a23affcd40ac74865d99637448346
content: 97e0223af6f3dd0d6274c5f2ce6b1aaf50c514d9e6f2573e80abd3b56d5ed8db
environment: local-macos-arm64
fingerprint: sha256:b715f7d51a1d415b857ae7ee964b8b3ee40e9123a147925fc3cb9ef8717d5c83
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:32:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8ca8901b2a, observed 2026-11-18T21:32:12Z. The raw log stays in the machine-local store.
