---
kind: evidence
id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-55bf2f
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
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:6d73b4a8aa9253177f9890f2bf1ff83038b97f8f3f1cab383ae4a070736f5bc5
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:46Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:46Z. The raw log stays in the machine-local store.
