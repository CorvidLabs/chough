---
kind: evidence
id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-68b196
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:7132da844e67fc75c87d0469f6e49bdfa716ee723cca91af6c4af02206924911
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:41Z. The raw log stays in the machine-local store.
