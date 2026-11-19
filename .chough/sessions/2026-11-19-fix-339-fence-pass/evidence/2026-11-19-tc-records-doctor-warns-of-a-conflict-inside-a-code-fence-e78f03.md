---
kind: evidence
id: 2026-11-19-tc-records-doctor-warns-of-a-conflict-inside-a-code-fence-e78f03
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
revision: eb828bedb11ed25410cf3961188a17cd516f2007
content: d8e7328df6aad290260d31c16bff20b466380b238047da93fd7359a1b61dbc71
environment: local-macos-arm64
fingerprint: sha256:94753204b0b2536910028619b8188dce73afd3c7d9ddd9cc8a40a1a076b0d195
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:39:02Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision eb828bedb11e, observed 2026-11-19T01:39:02Z. The raw log stays in the machine-local store.
