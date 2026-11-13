---
kind: evidence
id: 2026-11-13-tc-records-approval-as-the-human-29ac02
aliases: []
claim: "Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-approval-as-the-human]
case_digest: [fcb626c0dca4884cc45e23f20e6207f9c9069cfb78eac90f464383c6a8b7396b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - a_scope_gate_runs_the_approval_once_as_the_configured_human
  - a_review_gate_needs_a_verdict_and_runs_the_review_with_it
  - an_unknown_or_ambiguous_human_is_refused_before_anything_runs
  - the_project_file_overrides_the_user_file
  - no_source_names_a_person_as_the_default_human
exit_code: 0
duration_seconds: 2
revision: fd6efe7663471370b9119a34dc4d8f62f86aa487
content: e06fe9c67a7fccb02cb4bd517ea7fc260b3816a28367bcf2497ec25382c12c28
environment: local-macos-arm64
fingerprint: sha256:e986db611d3d11c7754b2366fc6e146b21518065a86065136484ff6d7b2b64fb
criteria: [FLOW-23, FLOW-24, FLOW-16]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T07:45:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision fd6efe766347, observed 2026-11-13T07:45:02Z. The raw log stays in the machine-local store.
