---
kind: evidence
id: 2026-11-12-tc-cli-accept-next-reruns-cases-not-current
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-next-reruns-cases-not-current]
case_digest: [45b18fb29f2e09ae23d6f4067c0435bc1c9a8f21d373a8f6984c2b442d24b767]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: 0c44e742c25af1a4bd717991b53b49f49342f300
content: 0d739975bf00ae228930e62297f6b0156014c9c12119a6f26cea471c15d2a9a0
environment: local-macos-arm64
fingerprint: sha256:58fdfed5e2d3546497216a4d05b065f7e1038c12a9a20cbb10cf4f78a6bac292
criteria: [FLOW-2, LIFE-7]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:54:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 0c44e742c25a, observed 2026-11-12T23:54:10Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate stdout ----
- thread 'at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate' (928570517) panicked at tests/next.rs:3034:9:
- assertion `left == right` failed: approved: Prepare the human's review, in order: run the affected cases with `chough cases run <case-id> --environment <env>`; open the review gate with `chough checkpoint --awaiting-human "Implementation review of f4-approved" --holds accept --drives review`; commit the source with Chough's records; then run `chough change check f4-approved`. The human reviews after that.
-   left: String("chough cases run <case-id> --environment <env>")
-  right: "chough cases run TC-cli-f4-rerun --environment local"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 33 filtered out; finished in 1.66s
