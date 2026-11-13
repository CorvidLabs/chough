---
kind: evidence
id: 2026-11-13-tc-cli-bug-observation-joins-the-session-8ec638
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:45f201a45b488707f08230141f216db7e32c3339a76b0f65e83026e408e930a7
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:09:35Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:09:35Z. The raw log stays in the machine-local store.
