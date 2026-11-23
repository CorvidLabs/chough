---
kind: evidence
id: 2026-11-23-tc-cli-gate-record-all-or-none
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_second_close_over_the_session_bound_refuses_the_whole_reply_with_nothing_written for TC-cli-gate-record-all-or-none: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-record-all-or-none]
case_digest: [e5e258441be39e237045926e35248647781cb60bf8578f211b775f1eb7779181]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_second_close_over_the_session_bound_refuses_the_whole_reply_with_nothing_written
exit_code: 0
duration_seconds: 1
revision: 039363a822d1908c5e6c11609c4e4bb743757b8a
content: 81904c057a9c9a851321f19cfcf6e327a282cd47dbf255f2d957e585d10bd651
environment: local-macos-arm64
fingerprint: sha256:80ab38c66f6b8a5cc6ce5585fd2f88bc1b58bc786d335301c1b82a26d94bf57e
criteria: [FLOW-16]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T18:08:49Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test gates -- --exact a_second_close_over_the_session_bound_refuses_the_whole_reply_with_nothing_written for TC-cli-gate-record-all-or-none: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 039363a822d1, observed 2026-11-23T18:08:49Z. The raw log stays in the machine-local store.
