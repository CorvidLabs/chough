---
kind: evidence
id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-83476b
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected]
case_digest: [086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - an_unreadable_skill_file_is_skipped_as_uninspected
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:b5ae604f6583229f6f28593e6b9a228502d4b99b21e95d9f7d7c970daf161288
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:27Z. The raw log stays in the machine-local store.
