---
kind: evidence
id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-40c735
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
revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
environment: local-macos-arm64
fingerprint: sha256:3ba8a3f36967369851eec56730a72ba444c4b2bbd54ab2675bdbe64ef08ea169
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:38:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f09cf6ba0f1e, observed 2026-11-19T05:38:09Z. The raw log stays in the machine-local store.
