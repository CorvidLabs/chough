---
kind: evidence
id: 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory]
case_digest: [28f1b67751997c8ab2317cd79e35944b6200f2e7c4ea66e276b90685dc273abd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
content: 1333c626b2a2b43886fb834482f27a2b7b636e7a76b89f250aab4ae71ddd56bd
environment: local-macos-arm64
fingerprint: sha256:792644d7e2ef59bd93975d2d885b8f3bf166496f7ae483275199118179c57067
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T02:09:01Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ea5224b381a4, observed 2026-11-18T02:09:01Z. The raw log stays in the machine-local store.
