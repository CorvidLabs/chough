---
kind: evidence
id: 2026-11-19-tc-cli-ship-names-an-unreadable-commit
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch for TC-cli-ship-names-an-unreadable-commit: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-names-an-unreadable-commit]
case_digest: [d5691cd7be664d02e1c2dadc748673d3cafc2a6a8af74e5de2cf70cd94ba8f8c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_with_an_unreadable_commit_names_fetch_refetch
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:498453e1659a47938936ecc87536028f3f541c30b4e0c11264aae54a0dd55803
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:08Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch for TC-cli-ship-names-an-unreadable-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:08Z. The raw log stays in the machine-local store.
