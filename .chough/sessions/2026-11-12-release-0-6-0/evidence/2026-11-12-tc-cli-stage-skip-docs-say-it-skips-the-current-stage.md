---
kind: evidence
id: 2026-11-12-tc-cli-stage-skip-docs-say-it-skips-the-current-stage
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-skip-docs-say-it-skips-the-current-stage]
case_digest: [3d6f5c15e943a0c26a1fb5d37e5be324a5a58f319890b5879729edb6ff45d13a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:fcbc7c8e67a0df443e384a1099a104417462c35d35e943b2ac22f3203a23193f
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:35Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:35Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design stdout ----
- thread 'stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design' (924181665) panicked at tests/docs.rs:1771:13:
- docs/cli.md does not say "skips the current stage"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.83s
