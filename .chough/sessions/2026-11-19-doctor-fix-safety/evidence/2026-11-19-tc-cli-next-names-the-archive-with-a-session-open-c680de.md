---
kind: evidence
id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-c680de
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
duration_seconds: 2
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:0946f43cff619e3b5bb8db7c8d505756587875a5890ae31d6b5827beed0b8cb7
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:25Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:25Z. The raw log stays in the machine-local store.
