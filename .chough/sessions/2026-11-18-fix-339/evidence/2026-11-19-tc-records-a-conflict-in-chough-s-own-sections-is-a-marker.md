---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-in-chough-s-own-sections-is-a-marker
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_a_conflict_inside_chough_s_own_fenced_sections for TC-records-a-conflict-in-chough-s-own-sections-is-a-marker: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-conflict-in-chough-s-own-sections-is-a-marker]
case_digest: [b0e098d8c24d22463c443894bfdd90f9441b9fa248a9b53f9f489cd687fa41a8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_reports_a_conflict_inside_chough_s_own_fenced_sections
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e72be7de72efed1c9917e9d18329bb4388c3c29a
content: ef0e3be4bc113f74006c5b98c137f39a04a285bdc4e2cec0d9126fc4328b6a36
environment: local-macos-arm64
fingerprint: sha256:d097a17e25b2a8858fa4d7cc17ccb397558e1bd6a00be364e0b0beb32794d858
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:21:39Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_a_conflict_inside_chough_s_own_fenced_sections for TC-records-a-conflict-in-chough-s-own-sections-is-a-marker: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e72be7de72ef, observed 2026-11-19T00:21:39Z. The raw log stays in the machine-local store.
