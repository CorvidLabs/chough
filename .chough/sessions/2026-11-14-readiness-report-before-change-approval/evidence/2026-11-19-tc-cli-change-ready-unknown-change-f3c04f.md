---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-unknown-change-f3c04f
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-unknown-change]
case_digest: [d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493]
command: [cargo, test, --locked, --test, ready, --, --exact, an_unknown_change_names_status]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
environment: local-macos-arm64
fingerprint: sha256:e2d71ef63bb0400ad6fce0fb61f9bcaaeb37e9054bee55a805a57d6187f9a7e4
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:09Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f0412cee9a8, observed 2026-11-19T05:11:09Z. The raw log stays in the machine-local store.
