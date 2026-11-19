---
kind: evidence
id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-bf2efa
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
duration_seconds: 10
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:2843629d8e0bb605e3119b4139928056896d265b4e180eec47121cb9248dac0e
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:12:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:12:41Z. The raw log stays in the machine-local store.
