---
kind: evidence
id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-fb684d
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
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:232699f9d40fe962335ba80cb2ef6c369f359a39f1c89295eaa884ebedc429f0
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:15:09Z. The raw log stays in the machine-local store.
