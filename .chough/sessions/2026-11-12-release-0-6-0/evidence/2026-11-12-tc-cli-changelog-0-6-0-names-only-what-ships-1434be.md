---
kind: evidence
id: 2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships-1434be
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
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:f84c196d178b8f448fe5d79c0f3a9bef07c8c339952442bec5d3ae0cd1f7763a
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:41Z. The raw log stays in the machine-local store.
