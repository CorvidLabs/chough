---
kind: evidence
id: 2026-11-12-tc-cli-bug-trigger-and-fix-807baa
aliases: []
claim: "Ran cargo test --locked --test bugs -- a_new_bugs_steps_start_with_the_labelled_trigger bug_update_fix_sets_the_fix_paragraph_and_logs_each_set verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision for TC-cli-bug-trigger-and-fix: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-trigger-and-fix]
case_digest: [9cf68f417f41c6ed78f176e37ec22b834adf0708462e09eb2868cde96534235c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - a_new_bugs_steps_start_with_the_labelled_trigger
  - bug_update_fix_sets_the_fix_paragraph_and_logs_each_set
  - verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision
exit_code: 0
duration_seconds: 1
revision: 06e0a1b0335e7c75b623567e77a8a9d38e087433
content: 10367cd0474ee6f739d7a93127ee5726844cc001fb604227fc2f374d06f2ca9c
environment: local-macos-arm64
fingerprint: sha256:1451023f1c8a82eea6023c976ae9bf0385dc661ae745c87f40fa0e02e732a0e1
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T17:47:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- a_new_bugs_steps_start_with_the_labelled_trigger bug_update_fix_sets_the_fix_paragraph_and_logs_each_set verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision for TC-cli-bug-trigger-and-fix: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 06e0a1b0335e, observed 2026-11-12T17:47:36Z. The raw log stays in the machine-local store.
