---
kind: evidence
id: 2026-11-18-tc-records-a-conflict-in-chough-s-own-sections-is-a-marker
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
revision: 05ea2811ead5c40449265fe3e68f3114375526eb
content: 668614d7d416992d94bdab698001370290f983469c669fc57b014ed26862ee3c
environment: local-macos-arm64
fingerprint: sha256:7481269298c3a51b70d41952dd62d665ae4e6becbebee36a0379df4ac6acc9cc
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:07:02Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_a_conflict_inside_chough_s_own_fenced_sections for TC-records-a-conflict-in-chough-s-own-sections-is-a-marker: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 05ea2811ead5, observed 2026-11-18T23:07:02Z. The raw log stays in the machine-local store.
