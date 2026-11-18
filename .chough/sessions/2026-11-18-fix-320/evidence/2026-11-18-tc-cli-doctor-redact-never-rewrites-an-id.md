---
kind: evidence
id: 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id
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
duration_seconds: 0
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 15b60cbbc36c600a2f34fb8dd5bdd2d2a758a03588e13446f8ce3cbbe60e1b33
environment: local-macos-arm64
fingerprint: sha256:fc908ad81c0db8360f414711adff7e2955a2d98a18d4bc58e94f5222461251fa
criteria: [LIFE-26]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:03:37Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T19:03:37Z. The raw log stays in the machine-local store.
