---
kind: evidence
id: 2026-11-13-tc-cli-bug-observation-joins-the-session-3c3074
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:667aea9361f8e68e347f9ccc65071973cf3ceee42459d73499f2414da98f2fbc
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:43:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:43:41Z. The raw log stays in the machine-local store.
