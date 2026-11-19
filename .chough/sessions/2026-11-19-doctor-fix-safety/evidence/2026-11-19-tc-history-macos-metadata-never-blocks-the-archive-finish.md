---
kind: evidence
id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-macos-metadata-never-blocks-the-archive-finish]
case_digest: [9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - macos_metadata_never_blocks_the_archive_finish
exit_code: 0
tests_run: 1
duration_seconds: 6
revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
environment: local-macos-arm64
fingerprint: sha256:46555f63b5fd7a2b516cdcfb012f296c263b202eb255ff2087a2edc22262008f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:34:10Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 73cf8c7aaca2, observed 2026-11-19T05:34:10Z. The raw log stays in the machine-local store.
