---
kind: evidence
id: 2026-11-18-tc-cli-ship-refuses-a-corrupt-history-00abbb
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
revision: 90ccb748a6bb192673f03dfa5f5013629bcd0b80
content: 913dda6e814a53c925d68b621761a7c42cfed4c078462cf8465ad5a8d99e3050
environment: local-macos-arm64
fingerprint: sha256:8518619dfbbaa7860c31898a38d90372a9f5ebce13e69b4fc27aa4b64e04a5e8
criteria: [LIFE-21]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:47:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 90ccb748a6bb, observed 2026-11-18T18:47:16Z. The raw log stays in the machine-local store.
