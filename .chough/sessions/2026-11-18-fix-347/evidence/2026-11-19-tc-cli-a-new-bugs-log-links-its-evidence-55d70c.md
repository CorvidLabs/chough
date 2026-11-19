---
kind: evidence
id: 2026-11-19-tc-cli-a-new-bugs-log-links-its-evidence-55d70c
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
duration_seconds: 1
revision: 5a1c2f7f01f76950328f80c957c55f3cf6964c8c
content: 842a9cb68cdccc8aefdccb00f11cc47666a31beeb8e507ad2344ef7b5b17ae92
environment: local-macos-arm64
fingerprint: sha256:3d87aa80bfc642c386ec017d3e53a61a5d3f75bf26a5d35d35a86c9b355ff1fc
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:21:36Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5a1c2f7f01f7, observed 2026-11-19T02:21:36Z. The raw log stays in the machine-local store.
