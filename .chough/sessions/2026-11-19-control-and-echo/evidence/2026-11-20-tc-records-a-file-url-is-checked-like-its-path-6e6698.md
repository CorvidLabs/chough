---
kind: evidence
id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-6e6698
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:d5d980f8aca412d5a1bdfc556fb95caedc9e4f4974fb0218b85578f754e586f2
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:38Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:38Z. The raw log stays in the machine-local store.
