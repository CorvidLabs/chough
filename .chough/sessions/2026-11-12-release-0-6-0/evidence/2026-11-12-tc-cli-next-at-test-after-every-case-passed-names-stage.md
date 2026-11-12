---
kind: evidence
id: 2026-11-12-tc-cli-next-at-test-after-every-case-passed-names-stage
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-test-after-every-case-passed-names-stage-advance]
case_digest: [b69ef226634ba10d38ffb9797057a4670a0a4ea6313c7090c55d9be7f79f4a1f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:3ba399a0ffeced47f45ac0d78ddcc5b33db3ccd1c75f0c338dfe3f3e03270da5
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:55:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:55:56Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run stdout ----
- thread 'at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run' (927166193) panicked at tests/next.rs:2928:5:
- assertion `left == right` failed: {
-   "action": "Continue test: Run the selected cases at test Select the cases this change affects with `chough cases select --requirement <id>` (or --criterion, --path), run each with `chough cases run <case-id> --environment <env>`, and record a failed run with `chough bug add`.",
-   "command": "chough cases run <case-id> --environment <env>",
-   "reason": "Recorded test evidence is needed before accept: run the affected cases or record a manual result with `chough evidence add`; native verification is not product proof."
- }
-   left: String("chough cases run <case-id> --environment <env>")
-  right: "chough stage advance"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 32 filtered out; finished in 1.17s
