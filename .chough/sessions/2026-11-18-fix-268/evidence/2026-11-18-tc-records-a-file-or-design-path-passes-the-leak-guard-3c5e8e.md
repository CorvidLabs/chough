---
kind: evidence
id: 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-3c5e8e
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
duration_seconds: 0
revision: 0ec498884fa5a68bf0c9528b4dc627a0fdf28751
content: 7433f506277b409e303be381d1401a0aba6095c5ff2812ba85b260fd89102238
environment: local-macos-arm64
fingerprint: sha256:bddfdb6f5bf851776a3010454bdf76b7b78114ab15ff1c5faf7d31d3dc7f25e5
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:54:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0ec498884fa5, observed 2026-11-18T15:54:57Z. The raw log stays in the machine-local store.
