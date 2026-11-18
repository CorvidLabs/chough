---
kind: evidence
id: 2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator-0e0d4a
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
duration_seconds: 1
revision: 05ea2811ead5c40449265fe3e68f3114375526eb
content: 668614d7d416992d94bdab698001370290f983469c669fc57b014ed26862ee3c
environment: local-macos-arm64
fingerprint: sha256:5cbb242c71c745f3e61a6203a0d408d4b0ee2aabc48accf6fa01bc4d13b544c8
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:06:58Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 05ea2811ead5, observed 2026-11-18T23:06:58Z. The raw log stays in the machine-local store.
