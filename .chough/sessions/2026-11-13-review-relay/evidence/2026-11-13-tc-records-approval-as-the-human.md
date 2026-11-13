---
kind: evidence
id: 2026-11-13-tc-records-approval-as-the-human
aliases: []
claim: "Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: failed (exit 101)."
result: failed
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
exit_code: 101
duration_seconds: 2
revision: da86ddcb2ed26059cef30effd419f2513a703f8f
content: 7d32c750c9713a5af53db6e07f5ef7e165d03878f169e5cea2e68190316f4702
environment: local-macos-arm64
fingerprint: sha256:98fc6e4d02e680ad2f1c29377aba991456162b412a4cb47944f7bf58cd5d8702
criteria: [FLOW-23, FLOW-24, FLOW-16]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - "expected output missing: \"test result: ok. 5 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T07:44:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision da86ddcb2ed2, observed 2026-11-13T07:44:22Z. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "test result: ok. 5 passed".

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
-      Running tests/humans.rs (target/debug/deps/humans-3f880b949495cf2a)
- error: test failed, to rerun pass `--test humans`
