---
kind: evidence
id: 2026-11-19-tc-records-doctor-warns-of-a-conflict-inside-a-code-fence
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
duration_seconds: 2
revision: e72be7de72efed1c9917e9d18329bb4388c3c29a
content: ef0e3be4bc113f74006c5b98c137f39a04a285bdc4e2cec0d9126fc4328b6a36
environment: local-macos-arm64
fingerprint: sha256:6fc682cd37534ad051a9206da87d4302b13a8eae55418ab93f7debd2fa55f01c
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:21:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e72be7de72ef, observed 2026-11-19T00:21:31Z. The raw log stays in the machine-local store.
