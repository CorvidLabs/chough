---
kind: evidence
id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-c3e9e9
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
duration_seconds: 1
revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
environment: local-macos-arm64
fingerprint: sha256:3302b0e1486ea87945659fffd01cc0672fea05a8a485fcb28a446dcb35ec8508
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:25:30Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0c31209bb80, observed 2026-11-19T15:25:30Z. The raw log stays in the machine-local store.
