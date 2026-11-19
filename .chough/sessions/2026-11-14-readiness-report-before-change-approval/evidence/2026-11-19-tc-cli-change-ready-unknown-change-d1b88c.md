---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-unknown-change-d1b88c
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
revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
environment: local-macos-arm64
fingerprint: sha256:94545703cf04411759eb2f8af20b63553bedd0a17a8fc5cc4b8eaf142516d3f2
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:08Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6ce76b8b0ab2, observed 2026-11-19T04:54:08Z. The raw log stays in the machine-local store.
