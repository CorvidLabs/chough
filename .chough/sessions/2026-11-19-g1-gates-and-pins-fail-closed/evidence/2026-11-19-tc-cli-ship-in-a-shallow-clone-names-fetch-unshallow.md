---
kind: evidence
id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow
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
duration_seconds: 1
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:63e5120ba99191957d4a40baf46c04620b5eb1f33971e963d86aa416d10d221e
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:27Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:27Z. The raw log stays in the machine-local store.
