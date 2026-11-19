---
kind: evidence
id: 2026-11-19-tc-records-doctor-warns-of-a-conflict-inside-a-code-fence-532975
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-warns-of-a-conflict-inside-a-code-fence]
case_digest: [81cbf5ac61bc0ca6ff2e697ad68a7436f0f2bf05f5af2888856a275d59b26a10]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_warns_of_a_conflict_inside_a_code_fence
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 08801812d7fe82d42f54451eb25087df5ddaeec6
content: 2b860426c9c1f962e92c5eaef48d4cf1ed31a739e1fc4da29359c8cb5baba200
environment: local-macos-arm64
fingerprint: sha256:263881dbc86acf78e46630b256a8576b85726114a891962568b1292cbcb1ab5b
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:41:26Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08801812d7fe, observed 2026-11-19T00:41:26Z. The raw log stays in the machine-local store.
