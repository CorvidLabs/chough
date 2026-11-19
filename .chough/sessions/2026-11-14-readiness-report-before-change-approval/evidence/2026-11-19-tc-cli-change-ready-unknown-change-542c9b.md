---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-unknown-change-542c9b
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
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:93b112f9b316288dbda965d94fccf9cb1b8d7482f5819fee39a2086c6d46ef5c
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:56Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:56Z. The raw log stays in the machine-local store.
