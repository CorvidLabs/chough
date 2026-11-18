---
kind: evidence
id: 2026-11-18-tc-cli-a-pinned-session-before-any-session-names-session-new
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
revision: af8ca8901b2a23affcd40ac74865d99637448346
content: 8497d83318cecdc87d4ae504d4419396a010b6f9157edeef384cf57564dbef06
environment: local-macos-arm64
fingerprint: sha256:518bffb1ef5ca0cd54438c425a80115ad23953fdc45948644ce41dec8050e0d3
criteria: [DRIVE-24]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:00:30Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_pinned_session_before_any_session_names_session_new for TC-cli-a-pinned-session-before-any-session-names-session-new: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8ca8901b2a, observed 2026-11-18T21:00:30Z. The raw log stays in the machine-local store.
