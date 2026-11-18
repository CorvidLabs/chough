---
kind: evidence
id: 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-0155b6
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-doctor-redact-never-rewrites-an-id]
case_digest: [2a2d44f7bb9ee9cba0c997e09b8e5602b59686686d29b91ca00db2e22b771997]
command: [cargo, test, --locked, --test, leaks, --, --exact, doctor_redact_never_rewrites_an_id]
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: c0c6180bea105d4e732380fe568d6fff2ba12ec2
content: 4bc998954c06b935c1a1d2f568666c78c58969ba9dda9327140366ba05502467
environment: local-macos-arm64
fingerprint: sha256:b4f3043510d8784161f65f4790e20531c0092d2a6b4b41443d038ee94b7d9eb7
criteria: [LIFE-26]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:58:59Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0c6180bea10, observed 2026-11-18T19:58:59Z. The raw log stays in the machine-local store.
