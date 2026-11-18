---
kind: evidence
id: 2026-11-18-tc-cli-a-leak-in-a-binary-metadata-file-is-found
aliases: []
claim: "Ran cargo test --locked --test special_files -- --exact a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten for TC-cli-a-leak-in-a-binary-metadata-file-is-found: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-leak-in-a-binary-metadata-file-is-found]
case_digest: [e8b15845b944e37e7929e33ccf0f45f82dde9e18cd739b1a520503ba9b20ca23]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - special_files
  - "--"
  - "--exact"
  - a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 3f8a8cb3a0677ff393daf067ed1e1391667e306a3792a743d9ea67651492f153
environment: local-macos-arm64
fingerprint: sha256:89b5acf37d9f6752b8cf6af9b3e53b1813dddd70ef3f1c403810e164b43c95e2
criteria: [LIFE-26]
requirements: [REQ-cli-068]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:54:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test special_files -- --exact a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten for TC-cli-a-leak-in-a-binary-metadata-file-is-found: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T17:54:28Z. The raw log stays in the machine-local store.
