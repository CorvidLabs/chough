---
kind: evidence
id: 2026-11-19-tc-records-a-fence-hides-a-conflict-only-when-closed-095bfc
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
revision: 4f74fa25cd491a82518ff5aead551e4814f3e043
content: 9f356f9326eedc20a0cb6c07e4dee6e014efc412386eb573f09bdec9a54197b6
environment: local-macos-arm64
fingerprint: sha256:445b578721f363783ce574d1d96d3ff269d6eb2366e12d80d5c99378511ec149
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:08:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f74fa25cd49, observed 2026-11-19T01:08:06Z. The raw log stays in the machine-local store.
