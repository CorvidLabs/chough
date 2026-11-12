---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-lessons-dated-form
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-a-lessons-dated-form]
case_digest: [c398f03757ff3c0531131399da18f71c70c7b7840dc1fb4f7e40c3128700c941]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - a_lessons_dated_form_is_documented_and_its_example_counts
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: de7893f8aad7c737300c7aa841f8737100d77f77
content: 57a5ce76a35a464f1e318ed9735b9a02f90f91f95eca257ebbe5e57f108f5279
environment: local-macos-arm64
fingerprint: sha256:7e818039a041b4635ef40c306c805c4884980995a4e3718e802dec72f752955e
criteria: [KNOW-12]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:14:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision de7893f8aad7, observed 2026-11-12T22:14:39Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_lessons_dated_form_is_documented_and_its_example_counts stdout ----
- thread 'a_lessons_dated_form_is_documented_and_its_example_counts' (925662526) panicked at tests/docs.rs:1842:9:
- skills/chough-workflow/references/accept.md does not say a lesson starts with its YYYY-MM-DD date
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s
