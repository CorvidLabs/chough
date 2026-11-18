---
kind: evidence
id: 2026-11-18-tc-records-migrate-refuses-conflict-markers
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
duration_seconds: 3
revision: 0b7ac77e7acf916d40a464035b1fa22c0b503d29
content: ce9fcb1fb2e58cb8ec126573304156028c1da80195af64bd728d412e98f4c48a
environment: local-macos-arm64
fingerprint: sha256:b6b3d1ec2a77c7d06395de07eb56286b9f037acba23200fb8bde4e5316592df7
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:15:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test migrate -- --exact migrate_refuses_a_plan_whose_files_would_hold_conflict_markers for TC-records-migrate-refuses-conflict-markers: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0b7ac77e7acf, observed 2026-11-18T19:15:00Z. The raw log stays in the machine-local store.
