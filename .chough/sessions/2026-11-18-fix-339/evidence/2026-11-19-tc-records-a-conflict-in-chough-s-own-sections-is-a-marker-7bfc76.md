---
kind: evidence
id: 2026-11-19-tc-records-a-conflict-in-chough-s-own-sections-is-a-marker-7bfc76
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
revision: 08801812d7fe82d42f54451eb25087df5ddaeec6
content: 2b860426c9c1f962e92c5eaef48d4cf1ed31a739e1fc4da29359c8cb5baba200
environment: local-macos-arm64
fingerprint: sha256:4396cc0938c04ef282a28e85d2ed2faba053020163d0f37f98ebb803e507aecf
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:41:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_a_conflict_inside_chough_s_own_fenced_sections for TC-records-a-conflict-in-chough-s-own-sections-is-a-marker: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08801812d7fe, observed 2026-11-19T00:41:24Z. The raw log stays in the machine-local store.
