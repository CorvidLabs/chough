---
kind: evidence
id: 2026-11-16-tc-cli-walk-ready-init-preserves
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact init_preserves_user_files_and_installs_a_complete_skill for TC-cli-walk-ready-init-preserves: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-walk-ready-init-preserves]
case_digest: [20d0ff4b51796a117809cce105515a667881e9296cae2a535a716a3ef13b4de3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - init_preserves_user_files_and_installs_a_complete_skill
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 844e012e0ad20d557919d6016b95564f98e7602c
content: 5c60974b1022830a04a02554b428bc6c5fd2c4e2acce6da4b53f39d7e8410bca
environment: local-macos-arm64
fingerprint: sha256:4f8251bcf55291bde1372eab06ff3bd6e004a9b1db05282f313becc0d08de092
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-16T03:35:34Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test cli -- --exact init_preserves_user_files_and_installs_a_complete_skill for TC-cli-walk-ready-init-preserves: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 844e012e0ad2, observed 2026-11-16T03:35:34Z. The raw log stays in the machine-local store.
