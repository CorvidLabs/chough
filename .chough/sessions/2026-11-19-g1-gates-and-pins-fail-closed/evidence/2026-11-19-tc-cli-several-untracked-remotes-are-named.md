---
kind: evidence
id: 2026-11-19-tc-cli-several-untracked-remotes-are-named
aliases: []
claim: "Ran cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from for TC-cli-several-untracked-remotes-are-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-several-untracked-remotes-are-named]
case_digest: [f05681de19c72c675616082a6d23120cf968ca7507dd2363d1a206db13f9ee9c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - several_untracked_remotes_are_named_to_choose_from
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:1538adf48fcae851c9c3d3c692b647d9eb34abe6bc610b8d805172971de67626
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:18Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from for TC-cli-several-untracked-remotes-are-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:18Z. The raw log stays in the machine-local store.
