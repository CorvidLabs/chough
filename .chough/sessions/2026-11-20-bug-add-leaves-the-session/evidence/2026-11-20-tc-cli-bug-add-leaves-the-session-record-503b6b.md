---
kind: evidence
id: 2026-11-20-tc-cli-bug-add-leaves-the-session-record-503b6b
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-leaves-the-session-record]
case_digest: [5865d20fc1880780d79a943f924712df017375d0e2d58e01bcfe04507f94bb22]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - bug_add_in_a_session_records_the_session_and_leaves_its_record
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:4cfbf534fcd93e24c40e6541ecd5097bea316ea200a5fd4da62ee532a3c6ed27
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:39Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:39Z. The raw log stays in the machine-local store.
