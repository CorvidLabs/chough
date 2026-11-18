---
kind: evidence
id: 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-5d82c1
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
revision: 6f5347343890308fc96ff050d004404d5c35bbe9
content: 33fab87e35740e5ff5f96d0121f4200d560a55a78ba7ed398d0cbf5795120a1a
environment: local-macos-arm64
fingerprint: sha256:6fc4690acf33e4c35a1f7e3d51448503db42839c2a3c530e0ef1a2e470324503
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:09:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6f5347343890, observed 2026-11-18T16:09:13Z. The raw log stays in the machine-local store.
