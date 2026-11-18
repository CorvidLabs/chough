---
kind: evidence
id: 2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence
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
revision: a6218613856f84a99d9f044d3b37a550e9600c3a
content: aa62b5a84b744615216780c98dcbf720e8e85f7344023c40d5b3fbc5c5e27697
environment: local-macos-arm64
fingerprint: sha256:d7cbf04843bfe0147a04a22e8ee41143f72edf8e9764eef2735d434210a9a970
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:09:18Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a6218613856f, observed 2026-11-18T23:09:18Z. The raw log stays in the machine-local store.
