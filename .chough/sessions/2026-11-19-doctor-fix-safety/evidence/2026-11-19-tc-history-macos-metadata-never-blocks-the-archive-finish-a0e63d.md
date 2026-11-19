---
kind: evidence
id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e63d
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
duration_seconds: 3
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:1fe355ce595d3654eeee5bacc4d953b21192fbd42c889058f90dd2d2a7379ab7
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:13:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:13:27Z. The raw log stays in the machine-local store.
