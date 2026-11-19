---
kind: evidence
id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-7964f9
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow]
case_digest: [4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:83542ed8f43704ab86d08d68f3866368a1b8abe440d3a9080d79662431988df2
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:44:17Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:44:17Z. The raw log stays in the machine-local store.
