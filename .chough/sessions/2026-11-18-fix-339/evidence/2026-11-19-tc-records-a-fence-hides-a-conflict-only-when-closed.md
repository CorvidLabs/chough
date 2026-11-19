---
kind: evidence
id: 2026-11-19-tc-records-a-fence-hides-a-conflict-only-when-closed
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fence-hides-a-conflict-only-when-closed]
case_digest: [5b9febc9e28afd4d7ac7f830c751fa4821b8b1384e1daef416793ac1cd2afe80]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_fence_hides_a_conflict_only_when_it_is_one_and_closes
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e72be7de72efed1c9917e9d18329bb4388c3c29a
content: ef0e3be4bc113f74006c5b98c137f39a04a285bdc4e2cec0d9126fc4328b6a36
environment: local-macos-arm64
fingerprint: sha256:64eef9043212424fee352e15842252ec7d632f13b0f88fdb7076398464a79fc8
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:21:37Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e72be7de72ef, observed 2026-11-19T00:21:37Z. The raw log stays in the machine-local store.
