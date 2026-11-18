---
kind: evidence
id: 2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving
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
duration_seconds: 1
revision: a6218613856f84a99d9f044d3b37a550e9600c3a
content: aa62b5a84b744615216780c98dcbf720e8e85f7344023c40d5b3fbc5c5e27697
environment: local-macos-arm64
fingerprint: sha256:37ed87c941310f466c64dd3903e52fc341d84de887bf433c024577e7168bd880
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:09:20Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a6218613856f, observed 2026-11-18T23:09:20Z. The raw log stays in the machine-local store.
