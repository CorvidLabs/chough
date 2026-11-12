---
kind: evidence
id: 2026-11-12-tc-cli-bug-observation-joins-the-session
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 9b9bc010bd5bdf3142ecaa618a7c1ca2e247ecfb
content: 6cca52c87760c87de5a9a7f3378fa6ce8bb85b07bd8f9709222683b7bb9f844c
environment: local-macos-arm64
fingerprint: sha256:54f7488e52812b24ccb9be27ce98b4d850752c938311476692702bf32a0b5a6b
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:50:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story for TC-cli-bug-observation-joins-the-session: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 9b9bc010bd5b, observed 2026-11-12T20:50:45Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story stdout ----
- thread 'a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story' (923757966) panicked at tests/bugs.rs:2200:5:
- assertion `left == right` failed
-   left: Array []
-  right: Array [String("2026-11-12-synthetic-checkout-charges-shipping-at-the-threshold")]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 30 filtered out; finished in 1.26s
