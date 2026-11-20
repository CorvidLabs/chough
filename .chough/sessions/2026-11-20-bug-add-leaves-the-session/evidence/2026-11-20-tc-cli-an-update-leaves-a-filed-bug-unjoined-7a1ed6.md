---
kind: evidence
id: 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined-7a1ed6
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-update-leaves-a-filed-bug-unjoined]
case_digest: [f4e6963593f982c31173b059805ac0f02375862d71524cccbaa96bfaa479d11c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:4bde595b4e3506af4501013bb899a4e47f44cb98f8f0026d712053126e793716
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:45Z. The raw log stays in the machine-local store.
