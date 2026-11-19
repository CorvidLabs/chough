---
kind: evidence
id: 2026-11-19-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving
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
revision: bacbabe4c9dc7161c22149715b36f2045c9d563f
content: 317a6d291d14e3e575b0dee9fe901b92570fd20f65091398b7f72c3d82b2e2e8
environment: local-macos-arm64
fingerprint: sha256:94f7dca068d7936be6c46050e35f0aa97c2538701afc5135b00c8e3e76bae0f3
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:00:21Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bacbabe4c9dc, observed 2026-11-19T02:00:21Z. The raw log stays in the machine-local store.
