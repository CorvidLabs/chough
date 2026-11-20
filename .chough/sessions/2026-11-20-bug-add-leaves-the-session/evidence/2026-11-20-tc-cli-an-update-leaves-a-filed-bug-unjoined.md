---
kind: evidence
id: 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined
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
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:12ebc4fcd0d7b751d96c4d66b74a382c25ca4b51812d1642ee225b0bbdffba85
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:31Z. The raw log stays in the machine-local store.
