---
kind: evidence
id: 2026-11-19-tc-history-next-names-finishing-the-archive-6264fb
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
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:d7199e90d5dd2c8fb0baab44079b0c92d7585d19b9db26272ecf51f2150d263f
criteria: [DRIVE-17]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:49Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:49Z. The raw log stays in the machine-local store.
