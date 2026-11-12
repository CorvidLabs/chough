---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-lessons-dated-form-3af8bf
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
duration_seconds: 1
revision: 70468c0a5ae1d54d71da2bb408d72b4af32c7447
content: b084fb6e7052b84f55c61038681d6e36b4e77bcce77324002537f5d357214a84
environment: local-macos-arm64
fingerprint: sha256:8dcf7cf0187f4c25fa4bd2a850c9fa71dca7f5caf8c3bcd2d9b24be10570316f
criteria: [KNOW-12]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:15:48Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 70468c0a5ae1, observed 2026-11-12T22:15:48Z. The raw log stays in the machine-local store.
