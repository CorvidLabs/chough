---
kind: evidence
id: 2026-11-19-tc-cli-a-new-bugs-log-links-its-evidence-d04268
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
revision: a58f6ca2f49e8558398be455056638882d5c49d9
content: 3c6940dd761fda83509301a761c2b6c01781b500181965677bc4a9fc7ec39c44
environment: local-macos-arm64
fingerprint: sha256:52b0d89e41afbc2e2b5421c362ec5bced5d3f3825aad06c70565564e6ed267ab
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:50:00Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a58f6ca2f49e, observed 2026-11-19T02:50:00Z. The raw log stays in the machine-local store.
