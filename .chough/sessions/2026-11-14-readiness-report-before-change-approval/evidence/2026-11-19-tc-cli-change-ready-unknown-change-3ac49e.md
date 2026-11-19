---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-unknown-change-3ac49e
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
revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
environment: local-macos-arm64
fingerprint: sha256:06bc2c6af51369c89e3909ab582df0869236e127d31a19c2d34503c7ac7e25be
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:04Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b9c1b4ad6f2, observed 2026-11-19T15:15:04Z. The raw log stays in the machine-local store.
