---
kind: evidence
id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-names-a-special-file-without-opening-it]
case_digest: [bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_names_a_special_file_without_opening_it
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:078ec0ae12ac48b8b317f9c9d114cddac019ab71e83494f0d32a7bab118c5c9d
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:48Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:48Z. The raw log stays in the machine-local store.
