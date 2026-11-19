---
kind: evidence
id: 2026-11-19-tc-cli-an-unreadable-change-folder-tree-refuses
aliases: []
claim: "Ran cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent for TC-cli-an-unreadable-change-folder-tree-refuses: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-unreadable-change-folder-tree-refuses]
case_digest: [1e6ffce9c44dd49dce72184a679c7212012cbb8970e65fe4a8d97b9e5b38921e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - an_unreadable_change_folder_tree_is_never_read_as_absent
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:1692fbe541ea1ba465f55420ad192cca8712ae32fe39aa3e363c621a40781daa
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:21Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent for TC-cli-an-unreadable-change-folder-tree-refuses: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:21Z. The raw log stays in the machine-local store.
