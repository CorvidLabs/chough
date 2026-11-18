---
kind: evidence
id: 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-file-or-design-path-passes-the-leak-guard]
case_digest: [3cf519108b057bcc98d52902e732ac75b7f9a832fe189bdd6acf1218c64af485]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_token_in_a_file_or_design_path_is_refused_at_the_write
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: be6c9f126e448bd33945a52261d55db2a9677d3d
content: 681fb0254b161e60bcdf7a5d0f9fbc464e49a23e49eaf5cda3f1458042f91a76
environment: local-macos-arm64
fingerprint: sha256:6fc156708e4704657e53b251b97acd89113ad8b05d11692e9263206b21edd7a5
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:38:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be6c9f126e44, observed 2026-11-18T15:38:35Z. The raw log stays in the machine-local store.
