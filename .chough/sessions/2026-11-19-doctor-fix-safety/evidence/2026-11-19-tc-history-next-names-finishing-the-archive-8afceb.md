---
kind: evidence
id: 2026-11-19-tc-history-next-names-finishing-the-archive-8afceb
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
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:22cadc73be9cc15456919162c88459417354384657204eac747082f1c4b75b12
criteria: [DRIVE-17]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:04Z. The raw log stays in the machine-local store.
