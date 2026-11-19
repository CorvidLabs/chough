---
kind: evidence
id: 2026-11-19-tc-cli-next-names-finishing-the-archive-a7d9b2
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
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:aad213421c893e00f8e5cf7b6effff3605ec8eae36dee3fbd2e1d914f97c3b65
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:50:03Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:50:03Z. The raw log stays in the machine-local store.
