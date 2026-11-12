---
kind: evidence
id: 2026-11-12-tc-cli-bug-observation-joins-the-session-60b2bd
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:bc32e0b2a5133d86b6002a668ec87f7cbc5000de28c92007599750b68f737159
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:22:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:22:07Z. The raw log stays in the machine-local store.
