---
kind: evidence
id: 2026-11-19-tc-cli-the-refetch-command-names-the-real-remote
aliases: []
claim: "Ran cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works for TC-cli-the-refetch-command-names-the-real-remote: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-refetch-command-names-the-real-remote]
case_digest: [6d88c02567ee7284d165d5993790c58a7d4545bb94089463166ea175c92f4886]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - the_refetch_command_names_the_real_remote_and_works
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:81a32ec5779f9b2e3c26e48cbab1578c9483f61e4a98a756bcbdbae2e8cf7a5a
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works for TC-cli-the-refetch-command-names-the-real-remote: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:15Z. The raw log stays in the machine-local store.
