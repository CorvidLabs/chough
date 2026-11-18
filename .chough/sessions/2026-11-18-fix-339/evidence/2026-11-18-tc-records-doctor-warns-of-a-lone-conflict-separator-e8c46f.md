---
kind: evidence
id: 2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator-e8c46f
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
content: 55c3fda98068cdcd631bca47109a154d0102300f8845de147a18d4d48de96e4a
environment: local-macos-arm64
fingerprint: sha256:47f2f7db990f3134eac769a1818e2726a1e65c463f805930793db739ed638141
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:33:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8ca8901b2a, observed 2026-11-18T21:33:08Z. The raw log stays in the machine-local store.
