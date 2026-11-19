---
kind: evidence
id: 2026-11-19-tc-cli-a-pinned-session-before-any-session-names-session-new
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
revision: bb64540c1b989f5f646990c51cdae0eb805078b7
content: 4974139543b4a7a776fbb21f9ccd50e23356a99eabd15cb10a5d2bce55729a17
environment: local-macos-arm64
fingerprint: sha256:c27770224f22174a908e3bda0677f5485fd8ac635f72150e87be94779ae1a7e6
criteria: [DRIVE-24]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:14:49Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_pinned_session_before_any_session_names_session_new for TC-cli-a-pinned-session-before-any-session-names-session-new: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bb64540c1b98, observed 2026-11-19T00:14:49Z. The raw log stays in the machine-local store.
