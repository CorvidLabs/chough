---
kind: evidence
id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph]
case_digest: [5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_body_gives_its_actual_line_its_own_paragraph
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
environment: local-macos-arm64
fingerprint: sha256:7b4b4b55225e791aeaafbaad35705f52d0c78c738f160b0b1b93b43e4900bc4c
criteria: [LIFE-2, LIFE-7, KNOW-30]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:12:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6b7cd4766a52, observed 2026-11-13T03:12:20Z. The raw log stays in the machine-local store.
