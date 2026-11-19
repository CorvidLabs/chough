---
kind: evidence
id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-dd0760
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-session-shelve-resolves-its-key]
case_digest: [0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - session_shelve_resolves_its_key_as_session_use_does
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 08399615be5c89eeecf91690cad2bfa741c2685c
content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
environment: local-macos-arm64
fingerprint: sha256:82e0fba5aeb59677c952f6e1273b87ee9bf1d63cb88841027fbd69881a4bb32f
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:04:19Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08399615be5c, observed 2026-11-19T17:04:19Z. The raw log stays in the machine-local store.
