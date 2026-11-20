---
kind: evidence
id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-bug-merge-never-shows-a-leaking-path]
case_digest: [774b3c328cf1ef1475cde9340b9edbf6fa619022f5f2b87d85cbc0e0b0afe416]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - bug_merge_never_shows_a_path_holding_a_token
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:7aa42caf17afd80cda1e946d249ba017265aae934514d4715e6c4c75d0b34cde
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:36Z. The raw log stays in the machine-local store.
