---
kind: evidence
id: 2026-11-12-tc-records-tree-and-readme
aliases: []
claim: "Ran cargo test --locked --test records -- --exact the_tree_holds_only_the_listed_paths for TC-records-tree-and-readme: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-tree-and-readme]
case_digest: [672613a0c9d3e40977b203c0d6136ff2d274a28416373489512ef0cdd069c8c6]
command: [cargo, test, --locked, --test, records, --, --exact, the_tree_holds_only_the_listed_paths]
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:e41a378e776408b148edc38925bc271c0e3e8750f707fbd04d9bf50f5086b6ee
criteria: [DRIVE-24, DRIVE-28, KNOW-32, KNOW-24]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:00:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact the_tree_holds_only_the_listed_paths for TC-records-tree-and-readme: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T15:00:12Z. The raw log stays in the machine-local store.
