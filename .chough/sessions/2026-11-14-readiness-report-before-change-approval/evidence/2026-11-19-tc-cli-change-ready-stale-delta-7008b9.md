---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-stale-delta-7008b9
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-stale-delta]
case_digest: [3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_stale_delta_is_named_until_the_body_is_rewritten
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
environment: local-macos-arm64
fingerprint: sha256:95eb432182ae247a95504579cd3f53e5370f51703757f8b62619a41f581f54c2
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:38:31Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e2c643fed4b, observed 2026-11-19T04:38:31Z. The raw log stays in the machine-local store.
