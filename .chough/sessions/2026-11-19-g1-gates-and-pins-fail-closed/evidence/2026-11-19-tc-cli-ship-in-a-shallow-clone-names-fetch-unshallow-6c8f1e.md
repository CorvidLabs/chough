---
kind: evidence
id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-6c8f1e
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
duration_seconds: 0
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:a53f513ff4781b61b7ddb8857c128cab9250dac9f5cb22abd8130cecfc6638a1
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:42Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:42Z. The raw log stays in the machine-local store.
