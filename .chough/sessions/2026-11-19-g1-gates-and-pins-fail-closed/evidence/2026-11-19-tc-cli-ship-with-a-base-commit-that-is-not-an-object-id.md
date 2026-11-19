---
kind: evidence
id: 2026-11-19-tc-cli-ship-with-a-base-commit-that-is-not-an-object-id
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on for TC-cli-ship-with-a-base-commit-that-is-not-an-object-id: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-with-a-base-commit-that-is-not-an-object-id]
case_digest: [59972040bf39a184f70acbc94a4d77422119eed98433a80726c6cd6c0c0a5d67]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_with_a_base_commit_that_is_not_an_object_id_goes_on
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:306bc6d20a7a7b045150716928e2f39e8724582bbf99c046e378f38450929c7a
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:11Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on for TC-cli-ship-with-a-base-commit-that-is-not-an-object-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:11Z. The raw log stays in the machine-local store.
