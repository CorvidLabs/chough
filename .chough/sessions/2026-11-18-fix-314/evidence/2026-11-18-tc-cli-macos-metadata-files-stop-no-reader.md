---
kind: evidence
id: 2026-11-18-tc-cli-macos-metadata-files-stop-no-reader
aliases: []
claim: "Ran cargo test --locked --test special_files -- --exact macos_metadata_files_in_chough_stop_no_reader for TC-cli-macos-metadata-files-stop-no-reader: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-macos-metadata-files-stop-no-reader]
case_digest: [7b22ff03ec7de8c1aee397381939fe7410980b759913abb361a81fd2c30e958b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - special_files
  - "--"
  - "--exact"
  - macos_metadata_files_in_chough_stop_no_reader
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 3f8a8cb3a0677ff393daf067ed1e1391667e306a3792a743d9ea67651492f153
environment: local-macos-arm64
fingerprint: sha256:ee195e255c725b586694cf4b4fa068a278376c727d99acc944148a31a9791f26
criteria: [DRIVE-14]
requirements: [REQ-cli-068]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:54:26Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test special_files -- --exact macos_metadata_files_in_chough_stop_no_reader for TC-cli-macos-metadata-files-stop-no-reader: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T17:54:26Z. The raw log stays in the machine-local store.
