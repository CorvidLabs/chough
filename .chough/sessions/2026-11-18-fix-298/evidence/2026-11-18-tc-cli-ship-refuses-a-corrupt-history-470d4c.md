---
kind: evidence
id: 2026-11-18-tc-cli-ship-refuses-a-corrupt-history-470d4c
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-refuses-a-corrupt-history]
case_digest: [e910d6ecf129ee761cade6e38e922c38b0af5780f188b4517ac83fed3f4e5fb6]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_refuses_a_history_with_a_change_both_open_and_archived
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 809082a6de20e7d00431449702138c470159c949
content: b6ebed039574cf57a518d7d7ebae8b7e0bb2a9b32fea7625cd00c48d1f65f3be
environment: local-macos-arm64
fingerprint: sha256:362451c2f503a06d43b46b38f4e4e9dd7ef896eb6038a072207f429a8e8b548f
criteria: [LIFE-21]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:48:29Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 809082a6de20, observed 2026-11-18T20:48:29Z. The raw log stays in the machine-local store.
