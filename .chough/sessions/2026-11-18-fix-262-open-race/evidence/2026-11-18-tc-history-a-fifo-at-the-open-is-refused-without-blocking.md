---
kind: evidence
id: 2026-11-18-tc-history-a-fifo-at-the-open-is-refused-without-blocking
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking for TC-history-a-fifo-at-the-open-is-refused-without-blocking: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-fifo-at-the-open-is-refused-without-blocking]
case_digest: [0452c2ce9fd21a8923c3cc1a7ad25d1c7fd34e6d7bdce53ad43def2369bf08b2]
command:
  - cargo
  - test
  - "--locked"
  - "--bin"
  - chough
  - "--"
  - "--exact"
  - records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking
exit_code: 0
duration_seconds: 0
revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
content: 66ba56fa05ba7a450c5a6b22557bceee3a2772e739378414f3d9918900f65083
environment: local-macos-arm64
fingerprint: sha256:234e90da0fd7ed614ef4e15337e9e500882571c640bab22aea6be0c93f45bde1
criteria: [DRIVE-14]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-18T02:11:26Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --bin chough -- --exact records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking for TC-history-a-fifo-at-the-open-is-refused-without-blocking: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision a58e0be0ed9d, observed 2026-11-18T02:11:26Z. The raw log stays in the machine-local store.
