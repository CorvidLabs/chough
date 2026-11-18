---
kind: evidence
id: 2026-11-18-tc-records-migrate-refuses-conflict-markers-dd4e5d
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact migrate_refuses_a_plan_whose_files_would_hold_conflict_markers for TC-records-migrate-refuses-conflict-markers: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-conflict-markers]
case_digest: [8c4fc9b94c918d4b85c0603b3ae348cdb221fa5f26b255109078bb7467e4d6b8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - migrate_refuses_a_plan_whose_files_would_hold_conflict_markers
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 5ae9b83ea3adb45f8f4e52bf5a5c5c5867c316f4
content: c7f5ad5c8d56d0dd0fc2b0413efd1a1b0bdc0a82299c500ec12b7eccae5d5009
environment: local-macos-arm64
fingerprint: sha256:dd30a18f53775ad9600519a0c46c2c6b9409eacacf37646663ce72136d7b434f
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:27:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test migrate -- --exact migrate_refuses_a_plan_whose_files_would_hold_conflict_markers for TC-records-migrate-refuses-conflict-markers: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5ae9b83ea3ad, observed 2026-11-18T20:27:16Z. The raw log stays in the machine-local store.
