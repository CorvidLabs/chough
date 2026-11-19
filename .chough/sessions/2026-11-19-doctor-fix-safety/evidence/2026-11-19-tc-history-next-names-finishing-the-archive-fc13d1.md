---
kind: evidence
id: 2026-11-19-tc-history-next-names-finishing-the-archive-fc13d1
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-next-names-finishing-the-archive]
case_digest: [070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a]
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
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:73e9af94c0716728a1aea6a8d91b8e7a96512df8ebf10de2fc5695a6fa5387ea
criteria: [DRIVE-17]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:37Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:37Z. The raw log stays in the machine-local store.
