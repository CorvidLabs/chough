---
kind: evidence
id: 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-47c83d
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
revision: fff07c6549e90059c7a5118f0d5117c5be2118d6
content: c50e6155d020c44852b155771fea62d3fc91376871af3eb4b87c9f70d13a9584
environment: local-macos-arm64
fingerprint: sha256:b88ac270b893199028e2996fe383911fcc9567d4710efb810b00c9adc76ccf72
criteria: [LIFE-26]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:25:59Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fff07c6549e9, observed 2026-11-18T19:25:59Z. The raw log stays in the machine-local store.
