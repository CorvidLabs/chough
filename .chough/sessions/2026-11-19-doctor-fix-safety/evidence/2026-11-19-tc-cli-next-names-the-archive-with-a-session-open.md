---
kind: evidence
id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-names-the-archive-with-a-session-open]
case_digest: [cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_with_a_session_open_names_an_unfinished_archive_first
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:b2ffdb7b8993ffc791c8fcdecb9ea1b1e777ceb375c0a7efc850028ccb26fdb2
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:32:03Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:32:03Z. The raw log stays in the machine-local store.
