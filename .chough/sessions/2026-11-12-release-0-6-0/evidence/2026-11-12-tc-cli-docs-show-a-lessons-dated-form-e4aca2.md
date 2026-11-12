---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-lessons-dated-form-e4aca2
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:0aadfdc38efdf6531b974c5c68c12645e63298050b5540538efd6690e645f454
criteria: [KNOW-12]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:24:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:24:55Z. The raw log stays in the machine-local store.
