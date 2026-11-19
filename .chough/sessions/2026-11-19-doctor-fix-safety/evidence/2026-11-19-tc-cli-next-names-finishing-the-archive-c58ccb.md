---
kind: evidence
id: 2026-11-19-tc-cli-next-names-finishing-the-archive-c58ccb
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
duration_seconds: 2
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:00f058fdc1fd125f3bd28ed8ead9b44100da6ba7411a0c631a1495978882fae9
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:15:04Z. The raw log stays in the machine-local store.
