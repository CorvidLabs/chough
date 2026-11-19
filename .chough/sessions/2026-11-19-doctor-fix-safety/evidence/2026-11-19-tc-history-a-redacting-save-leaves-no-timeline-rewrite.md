---
kind: evidence
id: 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite
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
duration_seconds: 1
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:37b75fb55912133e8c2963a5f5d16c073196a44e1cc1761fdfbbdc6a013e062f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:59Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:59Z. The raw log stays in the machine-local store.
