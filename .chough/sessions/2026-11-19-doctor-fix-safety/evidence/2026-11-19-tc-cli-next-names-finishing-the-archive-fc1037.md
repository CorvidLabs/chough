---
kind: evidence
id: 2026-11-19-tc-cli-next-names-finishing-the-archive-fc1037
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-names-finishing-the-archive]
case_digest: [cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_names_finishing_the_archive_until_the_folder_is_gone
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:f3e160828e50c4fe6d7edabadcba881009e6e1bc14c820d81da812b9f0d0b30a
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:20Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:20Z. The raw log stays in the machine-local store.
