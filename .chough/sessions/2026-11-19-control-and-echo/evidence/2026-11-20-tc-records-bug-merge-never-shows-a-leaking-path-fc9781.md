---
kind: evidence
id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-fc9781
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:2c7bdbd699e31159b5cc3ea6e3960868f3fcccebf1ecfe3b51f468b418a394cd
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:43Z. The raw log stays in the machine-local store.
