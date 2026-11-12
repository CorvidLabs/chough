---
kind: evidence
id: 2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships-81b73a
aliases: []
claim: "Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-changelog-0-6-0-names-only-what-ships]
case_digest: [9cfdfa019b484f365a9eaad6a8cf06597ee715aaf04b8a7883186168259ead0a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - changelog
  - "--"
  - "--exact"
  - the_0_6_0_also_list_names_nothing_0_6_0_removed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5f1c57c81296da2f1afd3180266844051680dcb3
content: d147d2faee47f0dbef9a0b63332105852ce094a35699f89beebee474e82c10c9
environment: local-macos-arm64
fingerprint: sha256:6fd0d8facd484ba3fa13b192f3b4be14f7d553a40d4ebc51a14215ce75f372f7
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:36:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5f1c57c81296, observed 2026-11-12T16:36:06Z. The raw log stays in the machine-local store.
