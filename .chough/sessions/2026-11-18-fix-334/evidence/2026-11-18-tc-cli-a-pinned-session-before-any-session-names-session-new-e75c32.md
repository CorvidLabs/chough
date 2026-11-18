---
kind: evidence
id: 2026-11-18-tc-cli-a-pinned-session-before-any-session-names-session-new-e75c32
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact a_pinned_session_before_any_session_names_session_new for TC-cli-a-pinned-session-before-any-session-names-session-new: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-pinned-session-before-any-session-names-session-new]
case_digest: [d918358f67e41b11e7835e5d5f228103baa6fb6362c9edda70cb8c829c1d28ee]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - a_pinned_session_before_any_session_names_session_new
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 125df52c5a08f0e249873fa6349cbb3749d70b3e
content: 34b2402e215af4ce0c920c13f3a30e74b7f86261b18e7eb729867bf6fb5e8301
environment: local-macos-arm64
fingerprint: sha256:b72b36c489c1b3ffa2d6a1bfd17fa24d82ff9f1c7330fb43d6f027fd636149bd
criteria: [DRIVE-24]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T22:40:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_pinned_session_before_any_session_names_session_new for TC-cli-a-pinned-session-before-any-session-names-session-new: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 125df52c5a08, observed 2026-11-18T22:40:41Z. The raw log stays in the machine-local store.
