---
kind: evidence
id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-d2664d
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
revision: 08399615be5c89eeecf91690cad2bfa741c2685c
content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
environment: local-macos-arm64
fingerprint: sha256:16ec371b31deb25fe39265fa333e4205d32540d9fea0723c48b8f59f5804e89e
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:04:22Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08399615be5c, observed 2026-11-19T17:04:22Z. The raw log stays in the machine-local store.
