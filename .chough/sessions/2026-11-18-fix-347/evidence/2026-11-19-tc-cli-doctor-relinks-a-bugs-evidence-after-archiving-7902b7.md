---
kind: evidence
id: 2026-11-19-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving-7902b7
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-doctor-relinks-a-bugs-evidence-after-archiving]
case_digest: [e61ed11316991f3577cc31ca63c67afbd50cacf5caf463e13f171e9c24ed405b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - doctor_relinks_a_bugs_evidence_after_its_session_is_archived
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: 5a1c2f7f01f76950328f80c957c55f3cf6964c8c
content: 842a9cb68cdccc8aefdccb00f11cc47666a31beeb8e507ad2344ef7b5b17ae92
environment: local-macos-arm64
fingerprint: sha256:122a53442d5120db6eb8c251e808a1afed58c99d34b2f852468dec0125eb9b05
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:21:43Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5a1c2f7f01f7, observed 2026-11-19T02:21:43Z. The raw log stays in the machine-local store.
