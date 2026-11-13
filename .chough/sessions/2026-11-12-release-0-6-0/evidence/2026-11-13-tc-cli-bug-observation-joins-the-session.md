---
kind: evidence
id: 2026-11-13-tc-cli-bug-observation-joins-the-session
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:c8dfd1d9af0abdb444fa661b6e628f310eadce7cad3885125745a57e6d5d44dc
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:18:27Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:18:27Z. The raw log stays in the machine-local store.
