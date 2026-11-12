---
kind: evidence
id: 2026-11-12-tc-cli-bug-observation-joins-the-session-736ba4
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-observation-joins-the-session]
case_digest: [f1b0bbacaf2a4c41822c0cbd02209d69e02a1e3ff188329c7f7138711d59eef8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 68c7a32a3480069a0dcc1f4683dea442e8bf9d17
content: cf1150d5ad99acc6a0f39be4458564929e73ae8b88de6a7d6a4e7121d6782ff1
environment: local-macos-arm64
fingerprint: sha256:5932a42dd64ae2ca262ca9b81c4eef2f11a756893148d55ccded1e64a019b7c3
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:52:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 68c7a32a3480, observed 2026-11-12T20:52:33Z. The raw log stays in the machine-local store.
