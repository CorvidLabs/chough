---
kind: evidence
id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-a25bd4
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
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:3b9ff86354d16ccdf76ae27ddacf68c6117fd09c6d4cedaa4403fd7b02a3d745
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:40Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:40Z. The raw log stays in the machine-local store.
