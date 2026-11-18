---
kind: evidence
id: 2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence-27a1ae
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-new-bugs-log-links-its-evidence]
case_digest: [1c5c1b9e28ec5f2688dbec67f18d4168dc13ad6b82d986fc0dbebcb47d047a9f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_new_bugs_log_links_its_evidence_relative_to_the_bug
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: c83c39131582362ac3efd62c9d6bcdbb29c3c752
content: 7c2a3a12315224b6a34ed50f2d67a36337458caeddaeb2c44db328777ca2cfe2
environment: local-macos-arm64
fingerprint: sha256:7219b7ab746ba6e488fca2db519ea14bd83c0322c241e4e7f97de75e249cfd24
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:27:01Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c83c39131582, observed 2026-11-18T23:27:01Z. The raw log stays in the machine-local store.
