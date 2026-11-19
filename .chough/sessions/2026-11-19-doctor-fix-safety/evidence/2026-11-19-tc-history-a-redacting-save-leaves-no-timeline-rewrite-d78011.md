---
kind: evidence
id: 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite-d78011
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
duration_seconds: 2
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:de0aefbe2f6292b91d82bc03a2b34c7d3038d3c6a941de4bf020b53f42deb91a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:11:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:11:36Z. The raw log stays in the machine-local store.
