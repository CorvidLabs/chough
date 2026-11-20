---
kind: evidence
id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-f5bb39
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-file-url-is-checked-like-its-path]
case_digest: [b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_file_url_is_checked_like_its_path_and_username_counts
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:4603d02700cfce87f818a3c8d2ef7b5af751d256f446b75befb54df74f4f3a1e
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:34:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:34:31Z. The raw log stays in the machine-local store.
