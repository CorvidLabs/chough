---
kind: evidence
id: 2026-11-18-tc-cli-ship-refuses-a-corrupt-history
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
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 41f74e92f14a5c2905315d56967a83e8483bb9fccb28bfbef33d954c147af427
environment: local-macos-arm64
fingerprint: sha256:b0d0e9f01c8dae808ff6bdc39ef781d48ac53cb5de6d08dc3440297d300ab623
criteria: [LIFE-21]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:44:53Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T18:44:53Z. The raw log stays in the machine-local store.
