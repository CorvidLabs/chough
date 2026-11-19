---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-unknown-change-4d4838
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
revision: e42c96fec77c56feff65256be9e1d5deb1f34742
content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
environment: local-macos-arm64
fingerprint: sha256:bb95bf0bd07f2819ce7d2692b490901a1d9dc90e57d6817a53d86f472eca044f
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:12:47Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e42c96fec77c, observed 2026-11-19T04:12:47Z. The raw log stays in the machine-local store.
