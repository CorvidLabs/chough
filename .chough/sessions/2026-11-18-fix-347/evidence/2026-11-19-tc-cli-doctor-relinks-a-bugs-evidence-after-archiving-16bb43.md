---
kind: evidence
id: 2026-11-19-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving-16bb43
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
revision: a58f6ca2f49e8558398be455056638882d5c49d9
content: 3c6940dd761fda83509301a761c2b6c01781b500181965677bc4a9fc7ec39c44
environment: local-macos-arm64
fingerprint: sha256:7038f4412143f3141a620f2f9c4733ada1b6865a7272122ee590ff8cba44c423
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:50:03Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a58f6ca2f49e, observed 2026-11-19T02:50:03Z. The raw log stays in the machine-local store.
