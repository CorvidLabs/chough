---
kind: evidence
id: 2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving-136a4e
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
duration_seconds: 2
revision: c83c39131582362ac3efd62c9d6bcdbb29c3c752
content: 7c2a3a12315224b6a34ed50f2d67a36337458caeddaeb2c44db328777ca2cfe2
environment: local-macos-arm64
fingerprint: sha256:bd7047c48287a26fb9f703d3c8b6ed9133951ddd730c54e3d0ba5b3ba3af24aa
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:27:06Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c83c39131582, observed 2026-11-18T23:27:06Z. The raw log stays in the machine-local store.
