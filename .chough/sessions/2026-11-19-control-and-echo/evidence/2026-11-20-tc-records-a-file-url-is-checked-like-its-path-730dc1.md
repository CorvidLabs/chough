---
kind: evidence
id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-730dc1
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
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:cd689ae962cd05e5f6c2a79a96a6033077625d2e16a5ed31fbeffb824c9905a5
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:39:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:39:09Z. The raw log stays in the machine-local store.
