---
kind: evidence
id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-6fa760
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:1f47c2b5c6332a42ad55bd433db467e33012cca3291c3e86af59c36cea720c89
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:56Z. The raw log stays in the machine-local store.
