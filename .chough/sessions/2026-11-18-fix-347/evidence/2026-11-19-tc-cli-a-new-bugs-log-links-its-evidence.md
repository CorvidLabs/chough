---
kind: evidence
id: 2026-11-19-tc-cli-a-new-bugs-log-links-its-evidence
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
revision: bacbabe4c9dc7161c22149715b36f2045c9d563f
content: 317a6d291d14e3e575b0dee9fe901b92570fd20f65091398b7f72c3d82b2e2e8
environment: local-macos-arm64
fingerprint: sha256:c34859a74888f873552b7372fb4cd43dbe1416aa1a0f6865be2206b8536aee91
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:00:18Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bacbabe4c9dc, observed 2026-11-19T02:00:18Z. The raw log stays in the machine-local store.
