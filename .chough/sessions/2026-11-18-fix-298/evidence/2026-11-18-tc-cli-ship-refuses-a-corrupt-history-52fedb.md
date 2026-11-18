---
kind: evidence
id: 2026-11-18-tc-cli-ship-refuses-a-corrupt-history-52fedb
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
revision: 47d29b9f6f45b0dfc799861ee071edbb2806c64d
content: c2fb03265e8e5d3afccc224c0dfe9239218b8201d4bffe5359b3a9031fe59a42
environment: local-macos-arm64
fingerprint: sha256:d37bc2be4aa7a6f2de8a3a15bbbb99f7bd6d36d60c8b798e330eecfbe52a73ee
criteria: [LIFE-21]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:30:30Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 47d29b9f6f45, observed 2026-11-18T19:30:30Z. The raw log stays in the machine-local store.
