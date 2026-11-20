---
kind: evidence
id: 2026-11-20-tc-history-a-redacting-save-leaves-no-timeline-rewrite
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-redacting-save-leaves-no-timeline-rewrite]
case_digest: [5ac9d84346c190942d5aaef827a471fad64fb3124d9e4fb564505b632568f91a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_redacting_save_leaves_no_timeline_rewrite
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:de4928d60123d2c3327e483eac4131c5412fc2d13bed5b9ba2b4a73f08d7b034
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:17Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:17Z. The raw log stays in the machine-local store.
