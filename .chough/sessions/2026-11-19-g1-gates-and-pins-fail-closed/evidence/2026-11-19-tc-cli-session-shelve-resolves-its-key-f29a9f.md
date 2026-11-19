---
kind: evidence
id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-f29a9f
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
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:58d25598a8bf97db4588bc03445cf7ce33ebeceb9ff8f80d679732564418e015
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:37Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:37Z. The raw log stays in the machine-local store.
