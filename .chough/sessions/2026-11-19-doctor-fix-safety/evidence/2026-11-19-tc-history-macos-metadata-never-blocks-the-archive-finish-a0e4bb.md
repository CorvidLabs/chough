---
kind: evidence
id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e4bb
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
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:f39163581b36f699157ed8c2bd584859baea4d42acec17666475ec4783560b51
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:34Z. The raw log stays in the machine-local store.
