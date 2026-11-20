---
kind: evidence
id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-5ae0f6
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:b11737f55faf8e52ac02f4902830e7772834117af93fa62ebfcaa5f345f0cd70
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:31Z. The raw log stays in the machine-local store.
