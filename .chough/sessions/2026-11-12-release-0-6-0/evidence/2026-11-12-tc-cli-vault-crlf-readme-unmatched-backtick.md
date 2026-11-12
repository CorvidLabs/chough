---
kind: evidence
id: 2026-11-12-tc-cli-vault-crlf-readme-unmatched-backtick
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list for TC-cli-vault-crlf-readme-unmatched-backtick: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-vault-crlf-readme-unmatched-backtick]
case_digest: [6f81a22fd5ef649f6fe027e53394a0e6ebebe8a5e8bce7e8086aa910e3b56acb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:58d144ec2952afb9a054e226a44791d2740d9cb483a30e7839511a9c5248d911
criteria: [KNOW-14, KNOW-15]
requirements: [REQ-cli-044]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:59:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test vault -- --exact crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list for TC-cli-vault-crlf-readme-unmatched-backtick: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T14:59:55Z. The raw log stays in the machine-local store.
