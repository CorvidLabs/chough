---
kind: evidence
id: 2026-11-18-tc-cli-ship-refuses-a-corrupt-history-e50125
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
revision: 985e8b442460c7716391135bfd7ecb3dc5ce4c1c
content: bccb92636e3f7d30d4682f5030c5074d44206ed1729ab0d3bbc4b40f42d5adec
environment: local-macos-arm64
fingerprint: sha256:35af4f458e7287888209af3a0ff298220cf0c14c9aeb0c31c97731cdf3426aed
criteria: [LIFE-21]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T22:45:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 985e8b442460, observed 2026-11-18T22:45:04Z. The raw log stays in the machine-local store.
