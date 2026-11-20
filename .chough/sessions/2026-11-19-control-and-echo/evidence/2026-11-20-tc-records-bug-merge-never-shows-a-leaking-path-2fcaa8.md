---
kind: evidence
id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-2fcaa8
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
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:60f6d429afe602781716fbfa562358a4b580c8c1f21e7b03870267adb02807f6
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:39:15Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:39:15Z. The raw log stays in the machine-local store.
