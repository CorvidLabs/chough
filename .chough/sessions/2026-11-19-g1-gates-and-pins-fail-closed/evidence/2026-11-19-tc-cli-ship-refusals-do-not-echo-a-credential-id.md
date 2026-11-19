---
kind: evidence
id: 2026-11-19-tc-cli-ship-refusals-do-not-echo-a-credential-id
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id for TC-cli-ship-refusals-do-not-echo-a-credential-id: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-refusals-do-not-echo-a-credential-id]
case_digest: [995f4394b5b03a4b0bc2867e427fc23e3d19cc20aa6af03daf90c30a5beae9e2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_refusals_do_not_echo_a_credential_id
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:a11e7bf8959060136381a0af42e9f64b7315e14ad6df1b41b3ee88e795990b6b
criteria: [DRIVE-3]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:05Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id for TC-cli-ship-refusals-do-not-echo-a-credential-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:05Z. The raw log stays in the machine-local store.
