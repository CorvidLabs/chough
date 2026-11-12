---
kind: evidence
id: 2026-11-12-tc-cli-bug-trigger-and-fix
aliases: []
claim: "Ran cargo test --locked --test bugs -- a_new_bugs_steps_start_with_the_labelled_trigger bug_update_fix_sets_the_fix_paragraph_and_logs_each_set verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision for TC-cli-bug-trigger-and-fix: failed (exit 101)."
result: failed
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
exit_code: 101
duration_seconds: 2
revision: caa29f5e704c512b9af03b0ea33bdaaff1fc962c
content: 9db289c4ce1f8d5f1ad79c80bb5b911b1f95526ca85ae2f905c26e24c0c96e59
environment: local-macos-arm64
fingerprint: sha256:56b469b3e78f89afbbbc8f81a4fd0beab6d10557a16a08c36d6204b9d9704d8b
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"test result: ok. 3 passed\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T17:45:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test bugs -- a_new_bugs_steps_start_with_the_labelled_trigger bug_update_fix_sets_the_fix_paragraph_and_logs_each_set verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision for TC-cli-bug-trigger-and-fix: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision caa29f5e704c, observed 2026-11-12T17:45:39Z. The raw log stays in the machine-local store.

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
-      Running tests/bugs.rs (target/debug/deps/bugs-6a9b3c5e82c110b4)
- error: test failed, to rerun pass `--test bugs`
